use crate::modules::*;
use polars::prelude::*;
use std::sync::mpsc;
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

    thread::scope(|scope| {
        let mut handles = vec![];
        for (init_value, tar_name) in target_var_init_values
            .iter()
            .zip(target_variable_names.iter())
        {
            let handle = scope.spawn(|| {
                let (max, min) = get_margine(
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
                );
                let df_tmp = match
                df!("Device_name"=>&[tar_name],"init"=>&[init_value],"min"=>&[min],"MAX"=>&[max]){
                    Ok(df) => df,
                    Err(error)=>{
                        panic!("df_tmp error\n{:?}",error)
                    },
                };
                tx.send(df_tmp).unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {   
            let _ = match handle.join(){
                Ok(_)=>print!(""),
                Err(why)=>panic!("{:?}",why)
            };
        }
    });
    for recieved in rx {
        result_dataframe.vstack_mut(&recieved).unwrap().rechunk();
        if result_dataframe.shape().0 == target_variables.shape().0{
            break;
        }
    }
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

// pub fn get_margines(circuit_netlist: &str,)
