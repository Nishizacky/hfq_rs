use crate::modules::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use polars::prelude::*;
use std::env;
use std::sync::{mpsc, Arc};
use std::thread;

pub fn get_margines(
    filename: &str,
    default_element_names: Vec<&str>,
    configs: MarginConfig,
    hfq: bool,
    rep: usize,
) -> DataFrame {
    //! マルチスレッドを使用します。計算時間がはやくなるけど最適化問題を解いているわけではないので出力された値は検証される必要があります
    //! そのうちD-Wave Leapとかと関連付けるかも？そうすれば最適化問題のいい感じの解を得られる。
    //!
    let default_dataframe = simulation(filename).unwrap();
    let target_variables = get_variables(filename, false).unwrap();
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(
        (target_variables.height()).try_into().unwrap(),
    ));
    pb.set_style(
        ProgressStyle::default_bar().template("[{elapsed_precise}]{spinner:.cyan}[{bar:40.cyan/blue}] {pos}/{len} main thread {msg}").unwrap().progress_chars("#>-"),
    );
    let target_variable_names = target_variables.column("Element_name").unwrap();
    let target_var_init_values = target_variables.column("default_value").unwrap();
    let sw_timings = get_switch_timings(
        &configs,
        &default_element_names,
        &default_dataframe.clone(),
        hfq,
    );
    let mut result_dataframe = DataFrame::empty();
    let (tx, rx) = mpsc::channel();

    let arc_m = Arc::new(m);
    thread::scope(|scope| {
        let mut handles = vec![];
        for (init_value, tar_name) in target_var_init_values.iter().zip(target_variable_names.iter())
        {
            let m = arc_m.clone();
            let handle = scope.spawn(|| {
                let (max, min) = get_margine_with_progress_bar(
                    filename,
                    &default_dataframe,
                    &sw_timings,
                    &target_variables,
                    init_value.to_string().parse::<f64>().unwrap().into(),
                    tar_name.to_string().as_str().into(),
                    &configs,
                    &default_element_names,
                    hfq,
                    rep,
                    m
                );
                let df_tmp = match
                df!("Device_name"=>&[tar_name],"init"=>&[init_value],"min"=>&[min],"MAX"=>&[max]){
                    Ok(df) => df,
                    Err(error)=>{
                        panic!("df_tmp error\n{:?}",error)
                    },
                };
                pb.inc(1);
                tx.send(df_tmp).unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = match handle.join() {
                Ok(_) => print!(""),
                Err(why) => panic!("{:?}", why),
            };
        }
    });

    for recieved in rx {
        result_dataframe.vstack_mut(&recieved).unwrap().should_rechunk();
        if result_dataframe.height() == target_variables.height() {
            break;
        }
    }
    let min_par = ((&(&(result_dataframe.column("min").unwrap()
        - result_dataframe.column("init").unwrap()).unwrap()
        / (result_dataframe.column("init").unwrap())).unwrap())
        * 100.0).rename("min%").rechunk();
    let max_par = ((&(&(result_dataframe.column("MAX").unwrap()
        - result_dataframe.column("init").unwrap()).unwrap()
        / result_dataframe.column("init").unwrap()).unwrap())
        * 100.0).rename("MAX%").rechunk();
    let average = (&((result_dataframe.column("min").unwrap()
        + result_dataframe.column("MAX").unwrap()).unwrap())
        / 2).rename("avg").rechunk();
    result_dataframe.with_column(average).unwrap();
    result_dataframe.with_column(min_par).unwrap();
    result_dataframe.with_column(max_par).unwrap();

    let range = (result_dataframe.column("MAX%").unwrap()
        - result_dataframe.column("min%").unwrap()).unwrap().rename("range%").rechunk();
    result_dataframe.with_column(range).unwrap();
    result_dataframe = result_dataframe.sort(
            ["range%"],
            SortMultipleOptions::new().with_order_descending(true),
        ).unwrap();
    pb.finish_with_message("done!");
    env::set_var("POLARS_FMT_MAX_ROWS", result_dataframe.height().to_string());
    result_dataframe
}

pub fn dataframe_to_json(df: DataFrame) -> String {
    //json方式にしてJSON.parseにすれば文字列のまま引き渡して処理することができる。
    let min_per_vec = df["min%"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let min_val_vec = df["min"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let max_per_vec = df["MAX%"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let max_val_vec = df["MAX"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let init_vec = df["init"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let avg_vec = df["avg"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let range_per_vec = df["range%"].f64().unwrap().to_vec_null_aware().left().unwrap();
    let mut name_vec = Vec::new();
    for name in df["Device_name"].str().unwrap().iter() {
        name_vec.push(name.unwrap());
    }
    let min_per_str = min_per_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let min_val_str = min_val_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let max_per_str = max_per_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let max_val_str = max_val_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let init_str = init_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let avg_str = avg_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let range_per_str = range_per_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
    let name_str = name_vec.iter().map(|s| format!("\"{}\"", s.to_string())).collect::<Vec<String>>().join(", ");

    format!("{{\"min_per\":[{min_per_str}],\"min_val\":[{min_val_str}],\"max_per\":[{max_per_str}],\"max_val\":[{max_val_str}],\"name\":[{name_str}],\"init\":[{init_str}],\"avg\":[{avg_str}],\"range_per\":[{range_per_str}]}}")
}