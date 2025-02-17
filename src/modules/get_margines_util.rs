use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use polars::prelude::*;
use regex::Regex;
use std::{fs::File, io::Read, process::exit, string::FromUtf8Error, sync::mpsc, thread};

use crate::modules::simulation::*;

use super::{MarginConfig, PI_RATIO};
/// このファイルでは各回路パーツを変動させた場合どこまで回路が予測通りに動作するのかを検証する関数をまとめています
/// 動作の検証は位相が変化するタイミングをあらかじめ収集してそのあとに回路パラメーターを変動させて得た位相出力がオリジナルと相違ないか計算します
/// 位相が変化するタイミングも自動で収集するコードが格納されています。そのコードの仕様がある程度わかっていないと理解しにくい関数かもしれません
/// 回路パラメータは一つずつ変更して検証しますが、時短のために他の回路パラメータは初期値のまま固定されるので完全な最適化を行いたい場合はこのコードを根本的に書き換える必要があります。その場合かなり検証時間がかかり最悪計算機がタイムアウトする可能性があるので議論の余地があります。
/// <使い方>
/// マージンを調べたいパラメータの行の最後に#variant_nameを書くことで関数はこの変数名でパラメータを認識します。初期値についてはその行に書かれているコードに記載された数字をそのまま採用します。(そうすることで通常の検証をしなおしたい際にコードを書き換えずに済むのでpythonのバージョンとは仕様を変えました。)
/// 同じ変数名が複数回登場する場合、初期値は一番最初に登場したものが採用されるため注意してください。
/// include機能についてはjosimに走らせたときにはじめて回収されます。この関数はjosimの起動前に処理を行う関数ですがinclude先のファイルを検査しないのでincludeで指示しているファイルは変数指定できません。注意してください。

fn get_switch_timing(
    config: &MarginConfig,
    element_name: &str,
    dataframe: &DataFrame,
    hfq: bool, //if false,it will be sfq
) -> Result<DataFrame, PolarsError> {
    //! 指定されたインデックスのデータを読み取り、どのタイミングでスイッチしているのかを計算して判定する。その結果を新しいデータフレームで出力する
    let pi: f64 = 3.14159265358979323846264338327950288;
    let phase = if hfq == true { pi } else { 2.0 * pi };
    let step_value = phase * PI_RATIO;
    let uppercase_element_names = String::from(element_name.to_uppercase());
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
pub fn get_switch_timings(
    config: &MarginConfig,
    judge_element_names: &Vec<&str>,
    df: &DataFrame,
    hfq: bool,
) -> Vec<DataFrame> {
    //! judge_element_namesとマージンを求めたいtargetの素子名は別物なので区別をしよう。
    let mut return_vec: Vec<DataFrame> = Vec::new();
    for &element in judge_element_names.iter() {
        return_vec.push(match get_switch_timing(&config, element, &df, hfq) {
            Ok(result) => result,
            Err(why) => panic!("switch timing error:\n{:?}", why),
        });
    }
    return_vec
}
fn fname_to_str(filename: &str) -> Result<String, FromUtf8Error> {
    //!ファイル名からStringに変換する
    let mut file = match File::open(filename) {
        Ok(result) => result,
        Err(why) => panic!("fname_to_str: {}", why),
    };
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();
    let file_text = String::from_utf8(data);
    file_text
}
pub fn get_variables(filename: &str, legacy: bool) -> Result<DataFrame, PolarsError> {
    //! legacy: 昔(種村さん)の記法で解析します。falseならば新しい記法での解析を実行します。これはよほど暇じゃない限り実装しないかも。
    let file_text = fname_to_str(filename).unwrap();
    if legacy {
        eprintln!("This function haven't been coded yet, sorry!");
        exit(1);
    } else {
        let re = Regex::new(r"(?<value>[\d.]+)\w*?\s*?#!(?<label>.+)\s*?\n").unwrap();
        //記法としては数字の後ろに#!を入れて、そのあと変数名を入れると解析
        //ex:  .param bias=20 #!var_name
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
            let tmp_df = df!("Element_name"=>&[v.0],"default_value"=>&[v.1])?;
            result_df.vstack_mut(&tmp_df)?;
        }
        result_df = result_df.unique_stable(
            Some(&[String::from("Element_name")]),
            UniqueKeepStrategy::First,
            None,
        )?;
        Ok(result_df)
    }
}
fn variable_changer(
    filename: &str,
    variable_df: &DataFrame,
    variable_target_name: &str,
    replace_number: f64,
) -> String {
    //! マージンを調べる際に変数の値を変えてくれるやつです。
    let file_text = fname_to_str(filename).unwrap();
    let reg_string = format!(
        r"(?<divider>[=\s]+)(?<value>[\d.]+)(?<prefix>\w?)\w*?\s*?#!\s*?{}\s*?\n",
        String::from(variable_target_name)
    )
    .replace("\"", "");
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
    let Some(caps) = re.captures(&file_text) else {
        return "".to_string();
    };
    let replace_str_tmp = replace_number.to_string();
    let divider = caps["divider"].to_string();
    let prefix = caps["prefix"].to_string();
    let replace_str =
        format!("{}{}{} #!{} \n",divider,replace_str_tmp,prefix,variable_target_name);
    let return_str = re.replace_all(&file_text, &replace_str);
    String::from(return_str)
}
fn switch_timing_comparator(
    default_dataframe: &DataFrame,
    target_dataframe: &DataFrame,
    config: &MarginConfig,
) -> bool {
    //! 元のデータと値を変えた場合とでスイッチするタイミングが同じなのか検証します。
    //! どちらもget_switch_timingで処理してあるDFでないとうまく動きません(それをちゃんと検知するように作るべきだろうか...?)
    //! とりあえずshapeで検知するようしにた。ベクトルはコンパクトになるけどスイッチタイミングも立派なデータなのでdataframeにしておくべきと判断。
    //! スイッチタイミングがあっているかどうかは時間と位相の行をそれぞれ引き算して特定の値以内であれば排除、排除してデータフレームが空っぽになればOK,空っぽにならなくて値が残っていれば一致していない箇所があるとして出力

    if !(default_dataframe.shape().1 == 2 && target_dataframe.shape().1 == 2) {
        eprintln!(
            "dataframe size error: default_dataframe {:?}, target_dataframe {:?}",
            default_dataframe.shape(),
            target_dataframe.shape()
        );
        exit(1)
    }
    if default_dataframe.shape() == target_dataframe.shape() {
        let default_df_names = default_dataframe.get_column_names();
        let target_df_names = target_dataframe.get_column_names();
        let time_series = Series::new(
            "time",
            (default_dataframe.column(default_df_names[0]).unwrap()
                - target_dataframe.column(target_df_names[0]).unwrap())
            .unwrap(),
        );
        let phase_series = Series::new(
            "phase",
            (default_dataframe.column(default_df_names[1]).unwrap()
                - target_dataframe.column(target_df_names[1]).unwrap())
            .unwrap(),
        );
        let sub_df = DataFrame::new(vec![time_series, phase_series]).expect("sub_df error");
        let time_mask = sub_df
            .column("time")
            .unwrap()
            .gt(config.pulse_error)
            .unwrap();
        let timing_sub_over = sub_df.filter(&time_mask).unwrap().height() == 0;
        let time_mask = sub_df
            .column("time")
            .unwrap()
            .lt(-config.pulse_error)
            .unwrap();
        let timing_sub_under = sub_df.filter(&time_mask).unwrap().height() == 0;
        let phase_mask = sub_df.column("phase").unwrap().gt(0.5).unwrap();
        let phase_sub_over = sub_df.filter(&phase_mask).unwrap().height() == 0;
        let phase_mask = sub_df.column("phase").unwrap().lt(-0.5).unwrap();
        let phase_sub_under = sub_df.filter(&phase_mask).unwrap().height() == 0;

        if timing_sub_over & timing_sub_under & phase_sub_over & phase_sub_under == true {
            return true;
        };
    }
    return false;
}
fn switch_timing_comparator_all(
    default_dfs: &Vec<DataFrame>,
    target_dfs: &Vec<DataFrame>,
    element_names: &Vec<&str>,
    config: &MarginConfig,
) -> bool {
    //! 複数のデバイスで比較したいときにまとめて処理する関数。比較する素子をソートしなおしてから比較する関数"switch_timing_comparator"を呼び出す。
    //! ベクトルのサイズは一致するかどうか検査するようにしていますが、一致しない名前のDataFrameがあったりMarginConfigがあっても検知できるように実装していないので想定外の動きをするかもしれません。
    if default_dfs.len() != target_dfs.len() || default_dfs.len() != element_names.len() {
        eprintln!(
            "\x1b[1;31merror\x1b[0m: Vec size mismatch, default = {}, target = {}, config = {}",
            default_dfs.len(),
            target_dfs.len(),
            element_names.len()
        );
        exit(1);
    }
    let mut sorted_default_dfs = default_dfs.clone();
    sorted_default_dfs.sort_by_key(|item| {
        let label = item.get_column_names()[1];
        element_names.iter().position(|&x| x == label)
    });
    let mut sorted_target_dfs = target_dfs.clone();
    sorted_target_dfs.sort_by_key(|item| {
        let label = item.get_column_names()[1];
        element_names.iter().position(|&x| x == label)
    });
    for i in 0..element_names.len() {
        if !switch_timing_comparator(&sorted_default_dfs[i], &sorted_target_dfs[i], &config) {
            return false;
        }
    }
    return true;
}
fn judge(
    filename: &str,
    sw_timing_dfs: &Vec<DataFrame>,
    variable_df: &DataFrame,
    element_name: &str,
    config: &MarginConfig,
    judge_element_names: &Vec<&str>,
    hfq: bool,
    replace_number: f64,
) -> bool {
    let sim_string = variable_changer(filename, variable_df, element_name, replace_number);
    let result_df = simulation(&sim_string).unwrap();
    //ここで-nanとなってるデータがあればこの時点でfalseを返すようにif文を作ること。
    let dtypes = result_df.dtypes();
    for dtype in dtypes {
        if dtype != DataType::Float64 {
            return false;
        }
    }
    let target_switch_timings = &get_switch_timings(config, judge_element_names, &result_df, hfq);
    switch_timing_comparator_all(
        sw_timing_dfs,
        target_switch_timings,
        judge_element_names,
        config,
    )
}

pub fn get_margine_with_progress_bar(
    filename: &str,
    _default_df: &DataFrame,
    sw_timing_dfs: &Vec<DataFrame>,
    variable_df: &DataFrame,
    initial_value: f64,
    element_name: &str,
    config: &MarginConfig,
    judge_element_names: &Vec<&str>,
    hfq: bool,
    rep: usize,
    m: Arc<MultiProgress>,
) -> (f64, f64) {
    //!マルチスレッドで生成される関数。
    //!
    if initial_value == 0.0 {
        panic!("default_value==0.0");
    }
    let (tx_max, rx_max) = mpsc::channel();
    let (tx_min, rx_min) = mpsc::channel();
    let pbar_style = ProgressStyle::with_template(
        "\t[{elapsed_precise}][{bar:20.red}] {pos}/{len} {msg}",
    )
    .unwrap();
    thread::scope(|scope| {
        let handle_max = scope.spawn(|| {
            let mut max = initial_value * 2.0;
            let mut delta = initial_value / 2.0;
            // 変数が2倍してもシミュレーションが通ればそのまま終了。通らなかったら1/2をした値をシミュの結果に応じて足したり引いたりする。
            let pb = m.add(ProgressBar::new(rep.try_into().unwrap()));
            pb.set_style(pbar_style.clone());
            pb.set_message(format!("finding max {}", element_name));
            let mut log: Vec<&str> = vec![];
            if judge(
                filename,
                sw_timing_dfs,
                variable_df,
                element_name,
                config,
                judge_element_names,
                hfq,
                max,
            ) {
            } else {
                max -= delta;
                delta /= 2.0;
                for _ in 0..rep - 1 {
                    if judge(
                        filename,
                        sw_timing_dfs,
                        variable_df,
                        element_name,
                        config,
                        judge_element_names,
                        hfq,
                        max,
                    ) {
                        max += delta;
                        log.push("+");
                    } else {
                        max -= delta;
                        log.push("-");
                    }
                    delta /= 2.0;
                    pb.inc(1);
                }
            }
            pb.finish_and_clear();
            tx_max.send(max).unwrap();
        });
        let handle_min = scope.spawn(|| {
            let pb = m.add(ProgressBar::new(rep.try_into().unwrap()));
            pb.set_style(pbar_style.clone());
            pb.set_message(format!("finding min {}", element_name));
            let mut min = initial_value / 2.0;
            let mut delta = initial_value / 4.0;
            let mut log: Vec<&str> = vec![];
            for _ in 0..rep {
                if judge(
                    filename,
                    sw_timing_dfs,
                    variable_df,
                    element_name,
                    config,
                    judge_element_names,
                    hfq,
                    min,
                ) {
                    min -= delta;
                    log.push("-");
                } else {
                    min += delta;
                    log.push("+");
                }
                delta /= 2.0;
                pb.inc(1);
            }
            pb.finish_and_clear();
            tx_min.send(min).unwrap();
        });
        let _ = match handle_max.join() {
            Ok(_) => print!(""),
            Err(why) => panic!("max finding error: {:?}", why),
        };
        let _ = match handle_min.join() {
            Ok(_) => print!(""),
            Err(why) => panic!("min finding error: {:?}", why),
        };
    });
    let max = rx_max.recv().unwrap();
    let min = rx_min.recv().unwrap();
    m.clear().expect("pbar closing error");
    return (max, min);
}
