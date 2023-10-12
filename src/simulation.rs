use chrono::prelude::*;
use polars::prelude::*;
use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::path::PathBuf;

///この関数はC++で作成されたjosim-cliを起動し、引数に入力された内容をjosim-cliに引き渡してその結果をpolarsライブラリで定義されているDataFrame変数に変換しreturnします。
/// 入力する引数は.jsmファイルのフルパス、あるいは回路情報(netlist)のプレーンテキストです。
/// netlistの文法についてはjosimの公式ドキュメントを参照してください。大体はspiceの文法にしたがっていますが一部特殊な記法があります。
/// .jsmのファイルはjosim-cliつまりこのrustがサブプロセスを実行する対象のターミナルからアクセスできる必要があります。
/// 総ステップ数が100000を超えるとタイムアウトする可能性があります。これはjosimの仕様です。netlistを書き換えましょう。
/// もしcsvやrawで出力されるファイルが欲しい場合この関数を書き換える必要があります。(今のところ)
pub fn simulation(circuit_netlist: &str,delete_csv:bool) -> Result<polars::prelude::DataFrame, PolarsError> {
    let output_fname = Local::now()
        .format("simresult_%Y_%b%d_%H:%M:%S.csv")
        .to_string();
    println!("filename is {}",output_fname);

    let arg_com: Vec<&OsStr> = if circuit_netlist.ends_with(".jsm") == true {
        vec![OsStr::new("-o"),OsStr::new(&output_fname),OsStr::new(circuit_netlist)]
    } else {
        vec![OsStr::new("-i")]
    };
    if circuit_netlist.ends_with(".jsm")==true {
        println!("-o mode");
    } else {
        println!("-i mode")
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
   
    let dataframe: DataFrame = if circuit_netlist.ends_with(".jsm") == true {
        match process.stdin.unwrap().write_all(circuit_netlist.as_bytes()) {
            Err(why) => panic!("couldn't write to josim-cli stdin: {}", why),
            Ok(_) => print!(""),
        }

        let mut stdout = Vec::new();
        process.stdout.unwrap().read_to_end(&mut stdout)?;

        let mut stderr = Vec::new();
        process.stderr.unwrap().read_to_end(&mut stderr)?;
        // io::stdout().write_all(&stdout)?;
        // io::stderr().write_all(&stderr)?;

        CsvReader::from_path(&output_fname)?
            .has_header(true)
            .finish()?
    } else {
        match process.stdin.unwrap().write_all(circuit_netlist.as_bytes()) {
            Err(why) => panic!("couldn't write to josim-cli stdin: {}", why),
            Ok(_) => print!(""),
        }

        let mut stdout = Vec::new();
        process.stdout.unwrap().read_to_end(&mut stdout)?;

        let mut stderr = Vec::new();
        process.stderr.unwrap().read_to_end(&mut stderr)?;

        // io::stderr().write_all(&stderr)?;

        let stdout_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&stdout);
        let result: Vec<&str> = stdout_str.split("100% Formatting Output\n").collect();
        // println!("{:?}", result);
        let csv_data: &String = &result[1].replace(",", "->");
        let csv_data: String = csv_data.replace("\n   ", "\n");
        let csv_data: String = csv_data.replace("    ", " ");
        let csv_data: String = csv_data.replace("   -", " -");
        let csv_data: String = csv_data.replace(" ", ",");
        CsvReader::new(io::Cursor::new(csv_data))
            .has_header(true)
            .finish()?
    };
    // println!("{:?}", dataframe);
    // io::stdout().write_all(&stdout)?;
    if delete_csv{
        let delete_file_path_buf = PathBuf::from(&output_fname);
        let delete_file_path = delete_file_path_buf.as_path();
        if fs::remove_file(delete_file_path).is_err(){
            println!("{}の削除に失敗しました。",output_fname);
        };
    }
    Ok(dataframe)
}