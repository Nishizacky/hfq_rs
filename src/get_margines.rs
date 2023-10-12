use polars::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::Read,
    process::exit,
    str::{self, Utf8Error},
    string::FromUtf8Error,
};

/// このファイルでは各回路パーツを変動させた場合どこまで回路が予測通りに動作するのかを検証する関数をまとめています
/// 動作の検証は位相が変化するタイミングをあらかじめ収集してそのあとに回路パラメーターを変動させて得た位相出力がオリジナルと相違ないか計算します
/// 位相が変化するタイミングも自動で収集するコードが格納されています。そのコードの仕様がある程度わかっていないと理解しにくい関数かもしれません...
/// 回路パラメータは一つずつ変更して検証しますが、時短のために他の回路パラメータは初期値のまま固定されるので完全な最適化を行いたい場合はこのコードを根本的に書き換える必要があります。その場合かなり検証時間がかかり最悪計算機がタイムアウトする可能性があるので議論の余地があります。
/// <使い方>
/// マージンを調べたいパラメータの行の最後に#variant_nameを書くことで関数はこの変数名でパラメータを認識します。初期値についてはその行に書かれているコードに記載された数字をそのまま採用します。(そうすることで通常の検証をしなおしたい際にコードを書き換えずに済むのでpythonのバージョンとは仕様を変えました。)
/// 同じ変数名が複数回登場する場合、初期値は一番最初に登場したものが採用されるため注意してください。
/// include機能についてはjosimに走らせたときにはじめて回収されます。この関数はjosimの起動前に処理を行う関数ですがinclude先のファイルを検査しないのでincludeで指示しているファイルは変数指定できません。注意してください。

pub struct MarginConfig {
    //素子名だけで組んでくれるマクロを作成しておくこと。
    // ここではシミュレーションの中で検査する時間帯や、誤差をどれだけ許容するかを指定します。
    pub ref_data_start_time: f64, //最初に回路の値がどうなっているのか初期状態を取得する必要があります。これはその参考値を取得する開始時間です
    pub ref_data_end_time: f64,   //初期状態の参考値を取得する終了時間です。
    pub pulse_error: f64, //処理対象の回路についてスイッチするタイミングの誤差を指定するものです。緩すぎると明らかな異常状態を見逃すリスクが上がります。
    // どの回路素子をこの関数が観察するか指定します。表記についてはjosimが出力する名前にあわせてください。(ex. P(number|number|...), V(number|...)など。)
    pub ref_element_name: String,
}
pub fn set_margin_config(element_name: &str) -> MarginConfig {
    let ref_data_start_time = 100e-12;
    let ref_data_end_time = 450e-12;
    let pulse_error = 150e-12;
    let ref_element_name = element_name.to_string();
    MarginConfig {
        ref_data_start_time,
        ref_data_end_time,
        pulse_error,
        ref_element_name,
    }
}
pub fn get_switch_timing(
    config: &MarginConfig,
    dataframe: &polars::prelude::DataFrame,
    hfq: bool, //if false,it will be sfq
) -> Result<polars::prelude::DataFrame, PolarsError> {
    //! 指定されたインデックスのデータを読み取り、どのタイミングでスイッチしているのかを計算して判定する。その結果を新しいデータフレームで出力する
    let pi: f64 = 3.14159265358979323846264338327950288;
    let step_value = if hfq == true { pi } else { 2.0 * pi };
    let uppercase_element_names = config.ref_element_name.clone();
    let column_names = vec![String::from("time"), uppercase_element_names.clone()];
    let mask_starttime = dataframe
        .column("time")?
        .gt(config.ref_data_start_time.clone())?;
    let starttime_cropped_df = dataframe.filter(&mask_starttime)?;
    if starttime_cropped_df.height() < 1 {
        eprintln!(
            "Starttime error: {} is greater than this dataframe",
            config.ref_data_start_time
        );
        exit(1);
    }
    let mask_endtime = starttime_cropped_df
        .column("time")?
        .lt(config.ref_data_end_time.clone())?;
    let initial_level_cropped_df = starttime_cropped_df.filter(&mask_endtime)?;
    if initial_level_cropped_df.height() == starttime_cropped_df.height() {
        eprintln!(
            "Endtime error: {} is greater than this dataframe",
            config.ref_data_end_time
        );
        exit(1);
    }
    if dataframe
        .select(&[uppercase_element_names.clone()])
        .is_err()
    {
        println!(
            "\x1b[1;31merror\x1b[0m: element_names \'{:?}\' does not exist in dataframe",
            uppercase_element_names
        );
        println!(
            "\x1b[1;32mnote\x1b[0m: here is the list of dataframe index\n{:?}",
            dataframe.get_column_names()
        );
        exit(1);
    }

    let mut result_df = dataframe.select(column_names.clone())?.head(Some(0));
    let mut cropped_df = dataframe.select(column_names.clone())?;
    let each_initial_level: f64 = initial_level_cropped_df
        .column(&uppercase_element_names)?
        .sum()
        .unwrap();
    let mut phase_level = each_initial_level / initial_level_cropped_df.height() as f64;
    phase_level += step_value;
    // continue;
    loop {
        let mask = cropped_df
            .column(&uppercase_element_names)?
            .gt(phase_level)?;
        cropped_df = cropped_df.filter(&mask)?;
        let add_df = cropped_df.head(Some(1));
        result_df = result_df.vstack(&add_df)?;
        phase_level += step_value;
        if cropped_df.height() < 1 {
            break;
        }
    }
    Ok(result_df.clone())
}
fn fname_to_str(filename: &str) -> Result<String, FromUtf8Error> {
    let mut file = File::open(filename).unwrap();
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();
    let file_text = String::from_utf8(data);
    file_text
}
pub fn get_variables(
    filename: &str,
    legacy: bool,
) -> Result<polars::prelude::DataFrame, PolarsError> {
    //! legacy: 昔(種村さん)の記法で解析します。falseならば新しい記法での解析を実行します。これはよほど暇じゃない限り実装しないかも。
    let file_text = fname_to_str(filename).unwrap();
    if legacy {
        eprintln!("This function haven't been coded yet, sorry!");
        exit(1);
    } else {
        let re = Regex::new(r"(?<value>[0-9]+)\s*?\w*?\s*?#!\s*?(?<label>.+)\s*?\n").unwrap();
        let default_data: Vec<(&str, f64)> = re
            .captures_iter(file_text.as_str())
            .map(|caps| {
                let label = caps.name("label").unwrap().as_str();
                let value = caps.name("value").unwrap().as_str().parse::<f64>().unwrap();
                (label, value)
            })
            .collect();
        let mut result_df = DataFrame::empty();
        for v in default_data {
            let tmp_df = df!("Element_name"=>&[v.0],"default_value"=>&[v.1]).unwrap();
            result_df.vstack_mut(&tmp_df).unwrap();
        }
        result_df = result_df
            .unique_stable(
                Some(&[String::from("Element_name")]),
                UniqueKeepStrategy::First,
            )
            .unwrap();
        Ok(result_df)
    }
}
pub fn variable_changer(
    filename: &str,
    variable_df: polars::prelude::DataFrame,
    variable_target_name: &str,
    ratio: f64,
) -> String {
    let file_text = fname_to_str(filename).unwrap();
    let target_string = String::from(r"(?<value>[0-9]+)\s*?\w*?\s*?\s#!\s*?(?<label>.+)\s*?\n");
    let reg_string = target_string.replace("(?<label>.+)", variable_target_name);
    let re = Regex::new(&reg_string).unwrap();
    if !re.is_match(&file_text) {
        eprintln!(
            "\x1b[1;31merror\x1b[0m: target_name '{}' does not exist in '{}'",
            variable_target_name, filename
        );
        exit(1);
    }
    let mask = variable_df
        .column("Element_name")
        .unwrap()
        .equal(variable_target_name)
        .unwrap();
    if mask.is_empty() {
        eprintln!(
            "\x1b[1;31merror\x1b[0m: target_name '{}' does not exist in DataFrame",
            variable_target_name
        );
        exit(1);
    }
    let target_number = variable_df
        .filter(&mask)
        .unwrap()
        .column("default_value")
        .unwrap()
        .str_value(0)
        .parse::<f64>()
        .unwrap();
    let replace_number = target_number * ratio;
    let replace_str_tmp = replace_number.to_string();
    let replace_str = replace_str_tmp + " #! " + variable_target_name + " \n";
    //valueの値だけを変更するにはどうすればいい？？
    let return_str = re.replace_all(&file_text, &replace_str);
    String::from(return_str)
}
pub fn switch_timing_comparator(
    default_dataframe: DataFrame,
    target_dataframe: DataFrame,
    config: MarginConfig,
) -> bool {
    if default_dataframe.shape() == target_dataframe.shape() {
        let default_series = default_dataframe.get_columns();
        let target_series = target_dataframe.get_columns();
        let subtract_time = default_series[0]
            .subtract(&target_series[0])
            .unwrap()
            .gt(config.pulse_error)
            .unwrap()
            .is_empty();
        let subtract_phase = default_series[1]
            .subtract(&target_series[1])
            .unwrap()
            .gt(0.5)
            .unwrap()
            .is_empty();
        if subtract_time == false && subtract_phase == false{
            return true;
        };
    }
    return false;
}
#[cfg(test)]
mod tests {
    use polars::{export::rayon::result, prelude::*};
    use regex::Regex;
    use std::{fs::File, io::Read, str};

    use crate::{
        get_switch_timing, get_variables, set_margin_config, simulation, variable_changer,
    };
    #[test]
    fn sw_test() {
        let filename = "/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm";
        let config = set_margin_config("P(49|X_SINK,48|X_SINK)");
        println!(
            "{:?}",
            get_switch_timing(&config, &simulation(filename, true).unwrap(), true)
        );
    }
    #[test]
    fn test() {
        let filename = "/home/nishizaki/hfq_rs/testdata.jsm";
        let config = set_margin_config("P(49|X_SINK,48|X_SINK)");
        println!(
            "{:?}",
            get_switch_timing(&config, &simulation(filename, true).unwrap(), true)
        );
        let variable_df = get_variables(filename, false).unwrap();
        let ratio = 1.0;
        let variable_target_name = "zero_in0";
        let circuit_netlist = variable_changer(filename, variable_df, variable_target_name, ratio);
        let dataframe = simulation(&circuit_netlist, true).unwrap();
        println!("{:?}", dataframe);
        let sw_timing_df = get_switch_timing(&config, &dataframe, true);
        println!("{:?}", sw_timing_df);
    }
    #[test]
    fn function_test() {
        let mut file = File::open("/home/nishizaki/hfq_rs/testdata.jsm").unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();
        let file_text = str::from_utf8(&data).unwrap();
        let re = Regex::new(r"(?<value>[0-9]+)\s#!\s*?(?<label>.+)\s*?\n").unwrap();
        let default_data: Vec<(&str, f64)> = re
            .captures_iter(file_text)
            .map(|caps| {
                let label = caps.name("label").unwrap().as_str();
                let value = caps.name("value").unwrap().as_str().parse::<f64>().unwrap();
                (label, value)
            })
            .collect();
        let mut result_df = DataFrame::empty();
        for v in default_data {
            let tmp_df = df!("Element_name"=>&[v.0],"default_value"=>&[v.1]).unwrap();
            result_df.vstack_mut(&tmp_df).unwrap();
        }
        result_df = result_df
            .unique_stable(
                Some(&[String::from("Element_name")]),
                UniqueKeepStrategy::First,
            )
            .unwrap();
        let result_num = result_df
            .column("default_value")
            .unwrap()
            .str_value(0)
            .parse::<f64>()
            .unwrap();
        println!("number is {:?}\n df is below {:?}", result_num, result_df);
    }
}

// pub fn get_margines(circuit_netlist: &str,)
