use crate::get_margines_util::*;
use crate::simulation;
use polars::prelude::*;
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
    let default_dataframe = simulation(filename, true).unwrap();
    let target_variables = get_variables(filename, false).unwrap();
    let target_variable_names = target_variables.column("Element_name").unwrap();
    let target_var_init_values = target_variables.column("defalut_value").unwrap();
    let sw_timings = get_switch_timings(
        &configs,
        &default_element_names,
        &default_dataframe.clone(),
        true,
    );
    let result_dataframe = DataFrame::empty();

    for (init_value, tar_name) in target_var_init_values
        .iter()
        .zip(target_variable_names.iter())
    {
        thread::scope(|__s| {
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
            let df_tmp =
                df!("Device_name"=>&[tar_name],"init"=>&[init_value],"min"=>&[min],"MAX"=>&[max])
                    .unwrap();
            let _ = result_dataframe.vstack(&df_tmp);
        });
    }
    return result_dataframe;
}
#[cfg(test)]
mod tests {
    use crate::MarginConfig;
    use crate::get_margines::get_margines;
    #[test]
    fn get_margine_test() {
        let filename = "/home/nishizaki/hfq_rs/part2.jsm";
        let config = MarginConfig::new();
        println!(
            "{:?}",
            get_margines(filename, vec!["P(49|X_sink,48|X_sink)"], config, true, 8)
        );
    }
}

// pub fn get_margines(circuit_netlist: &str,)
