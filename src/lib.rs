pub mod modules;

/*
構想：modで一つのnetlist関していろいろ操作ができるように関数間でデータを共有できるようにしておきたい
共有させておきたい関数: simulation, plot, critical_margine
*/

#[cfg(test)]
mod tests {
    use crate::modules::*;
    use polars::prelude::*;
    #[test]
    fn simulation_test() {
        let filename = "tests/jtl.jsm";
        print!("{:?}", simulation(filename));
    }

    #[test]
    fn margine_check_test() {
        let filename = "tests/part3.jsm";
        let config = MarginConfig::new();
        println!(
            "{:?}",
            get_margines(filename, vec!["P(49|X_sink,48|X_sink)"], config)
        );
    }

    #[test]
    fn fn_test() {
        let filename = "tests/xor.jsm";
        let config = MarginConfig::new();
        let df = get_margines(
            filename,
            vec!["P(3|X5|X34)", "P(3|X5|X44)", "P(3|X5|X54)", "P(3|X5|X71)"],
            config,
        );
        println!("{:?}", df);
        println!("{}", dataframe_to_json(df))
    }
}
