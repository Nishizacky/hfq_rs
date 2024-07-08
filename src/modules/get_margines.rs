use crate::modules::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use polars::prelude::*;
use std::sync::{mpsc,Arc};
use std::thread;

pub fn get_margines(
    filename: &str,
    default_element_names: Vec<&str>,
    configs: MarginConfig,
    hfq: bool,
    rep: usize,
) -> DataFrame {
    //! 一応マルチスレッドを使用します。計算時間がはやくなるけど最適化問題を解いているわけではない。
    //! そのうちD-Wave Leapとかと関連付けるかも？そうすれば最適化問題のいい感じの解を得られる。
    //!
    let default_dataframe = simulation(filename).unwrap();
    let target_variables = get_variables(filename, false).unwrap();
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new((target_variables.height()*2).try_into().unwrap()));
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}][{bar:40.cyan/blue}] {pos}/{len} main thread {msg}")
        .unwrap()
        .progress_chars("#>-"));
    let target_variable_names = target_variables.column("Element_name").unwrap();
    let target_var_init_values = target_variables.column("default_value").unwrap();
    let sw_timings = get_switch_timings(
        &configs,
        &default_element_names,
        &default_dataframe.clone(),
        true,
    );
    let mut result_dataframe = DataFrame::empty();
    let (tx, rx) = mpsc::channel();
    
    let arc_m = Arc::new(m);
    println!("hi");
    thread::scope(|scope| {
        let mut handles = vec![];
        for (init_value, tar_name) in target_var_init_values
            .iter()
            .zip(target_variable_names.iter())
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
        result_dataframe.vstack_mut(&recieved).unwrap().rechunk();
        if result_dataframe.shape().0 == target_variables.shape().0 {
            break;
        }
    }
    pb.finish_with_message("done!");
    result_dataframe
}
#[cfg(test)]
mod tests {
    use crate::modules::*;
    #[test]
    fn get_margine_test() {
        let filename = "/home/nishizaki/hfq_rs/part3.jsm";
        let config = MarginConfig::new();
        println!(
            "{:?}",
            get_margines(filename, vec!["P(49|X_sink,48|X_sink)"], config, true, 8)
        );
    }
}
