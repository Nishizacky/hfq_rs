use chrono::prelude::*;
use polars::prelude::*;
use regex::Regex;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{Cursor, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

///この関数はC++で作成されたjosim-cliを起動し、引数に入力された内容をjosim-cliに引き渡してその結果をpolarsライブラリで定義されているDataFrame変数に変換しreturnします。
/// 入力する引数は.jsmファイルのフルパス、あるいは回路情報(netlist)のプレーンテキストです。
/// netlistの文法についてはjosimの公式ドキュメントを参照してください。大体はspiceの文法にしたがっていますが一部特殊な記法があります。
/// .jsmのファイルはjosim-cliつまりこのrustがサブプロセスを実行する対象のターミナルからアクセスできる必要があります。
/// 総ステップ数が100000を超えるとタイムアウトする可能性があります。これはjosimの仕様です。netlistを書き換えましょう。
/// もしcsvやrawで出力されるファイルが欲しい場合この関数を書き換える必要があります。(今のところ)
pub fn simulation_with_csvfile(circuit_netlist: &str, delete_csv: bool) -> Result<DataFrame, PolarsError> {
    let output_fname = Local::now()
        .format("/tmp/hfq_rs/simresult_%Y_%b%d_%H:%M:%S.csv")
        .to_string();
    // println!("filename is {}",output_fname);

    let arg_com: Vec<&OsStr> = if circuit_netlist.ends_with(".jsm") == true {
        vec![
            OsStr::new("-o"),
            OsStr::new(&output_fname),
            OsStr::new(circuit_netlist),
        ]
    } else {
        vec![OsStr::new("-i")]
    };
    if circuit_netlist.ends_with(".jsm") == true {
        // println!("filepath_input");
    } else {
        // println!("stdin_input")
    }
    let process = match Command::new("josim-cli")
        .args(&arg_com)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn josim-cli: {}", why),
        Ok(process) => process,
    };

    if circuit_netlist.ends_with(".jsm") == true {
        match process.stdin.unwrap().write_all(circuit_netlist.as_bytes()) {
            Err(why) => panic!("couldn't write to josim-cli stdin: {}", why),
            Ok(_) => print!(""),
        }

        let mut stdout = Vec::new();
        process.stdout.unwrap().read_to_end(&mut stdout)?;

        let mut stderr = Vec::new();
        process.stderr.unwrap().read_to_end(&mut stderr)?;
    } else {
        //.fileがないか追跡する。あればファイル名を削除。そして出力ファイル名をtranの次の行に加筆する。これはファイル本体の文字列ではなくコピーした文字列だけ変わるので本体への影響は考えなくてもいい。
        let find_re = Regex::new(r".file.+\n").unwrap();
        let circuit_netlist_str = find_re.replace(circuit_netlist, "\n").to_string();
        let re = Regex::new(r"(.tran.+\n)").unwrap();
        let cap = re
            .captures(&circuit_netlist_str)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let format = format!("{}.file {}\n", cap, output_fname);
        let replace = format.replace("\"", "");
        let input_str = re.replace(&circuit_netlist_str, replace);
        match process.stdin.unwrap().write_all(input_str.as_bytes()) {
            Err(why) => panic!("couldn't write to josim-cli stdin: {}", why),
            Ok(_) => print!(""),
        }

        let mut stdout = Vec::new();
        process.stdout.unwrap().read_to_end(&mut stdout)?;

        let mut stderr = Vec::new();
        process.stderr.unwrap().read_to_end(&mut stderr)?;
    };
    // let dataframe = read_csv_file(output_fname);
    let dataframe = CsvReader::from_path(&output_fname)?
        .has_header(true)
        .finish()?;
    if delete_csv {
        let delete_file_path_buf = PathBuf::from(&output_fname);
        let delete_file_path = delete_file_path_buf.as_path();
        if fs::remove_file(delete_file_path).is_err() {
            println!("{}の削除に失敗しました。", output_fname);
        };
    }
    Ok(dataframe)
}

pub fn simulation(circuit_netlist: &str) -> Result<DataFrame, PolarsError> {
    //!　ファイルの出力を標準出力にしてそれを拾い、csv方式に変換してdataframeを作成する。この際ファイルを作成せず、ファイルストリームをストレージを使わず作成するためメモリ不足の場合は十分な動作が期待できない。
    let arg_com: Vec<&OsStr> = if circuit_netlist.ends_with(".jsm") {
        // vec![OsStr::new(circuit_netlist)]
        vec![OsStr::new("-i")]
    } else {
        vec![OsStr::new("-i")]
    };
    let mut input = String::new();
    if circuit_netlist.ends_with(".jsm"){
        let mut f = File::open(circuit_netlist).expect("file not found");
        f.read_to_string(&mut input).expect("something went wrong reading the file ");
    }else{
        input = circuit_netlist.to_string();
    }
    let re = Regex::new(r"#!.+\n").unwrap();
    let input = re.replace_all(&input,"\n");
    let process = match Command::new("josim-cli")
        .args(&arg_com)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn josim-cli: {}", why),
        Ok(process) => process,
    };
    match process.stdin.unwrap().write_all(input.as_bytes()) {
        Err(why) => panic!("couldn't write to josim-cli stdin: {}", why),
        Ok(_) => print!(""),
    }
    
    let mut stdout = Vec::new();
    process.stdout.unwrap().read_to_end(&mut stdout)?;

    let mut stderr = Vec::new();
    process.stderr.unwrap().read_to_end(&mut stderr)?;

    let stdout_str = String::from_utf8(stdout).unwrap();
    if !stdout_str.contains("100% Formatting Output\n"){
        panic!("{}\n\n Matrix is singular. Matrix will have no solution.
        Please check the components in the netlist. ",input);
    }
    let filestream = raw_to_filestream(stdout_str);
    CsvReader::new(filestream).has_header(true).finish()
}

fn raw_to_filestream(input: String) -> std::io::Cursor<Vec<u8>> {
    let slice: Vec<&str> = input.split("100% Formatting Output\n").collect();
    let return_str = slice[1];

    let re = Regex::new(r"\n +").unwrap();
    let return_str = re.replace_all(return_str, "\n").to_string();

    let re = Regex::new(r" +").unwrap();
    let return_str = re.replace_all(&return_str, ",");
    // println!("{:?}",return_str);
    
    // Cursor を使用してバッファを持つファイルストリームを作成
    let content_bytes = return_str.as_bytes();
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_all(content_bytes).unwrap();
    cursor
}

#[cfg(test)]
mod tests {
    use crate::modules::simulation::{simulation,simulation_with_csvfile};
    #[test]
    fn file_input_test() {
        let filename = "/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm";
        print!("{:?}", simulation_with_csvfile(filename,true));
    }
    #[test]
    fn direct_input_test() {
        let content = include_str!("/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm");
        print!("{:?}", simulation(content));
    }
}
