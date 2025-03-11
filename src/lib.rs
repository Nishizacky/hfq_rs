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
    fn df_test() {
        let _data = df!("Device_name"=>&["L3","BIAS","L2","R2","SQUID2","R1","SQUID7","L1","SQUID5","SQUID6","SQUID4","SQUID3","SQUID8"],
                "init"=>&[2.0,0.5,9.0,42.0,0.608,80.0,0.608,13.8,0.488,0.548,0.668,0.578,0.728],
                "min"=>&[0.003906,0.061523,1.599609,8.449219,0.0011875,35.46875,0.0011875,5.094141,0.000953,0.00107,0.515352,0.493332,0.646953],
                "MAX"=>&[4.0,1.0,18.0,84.0,0.966625,160.0,0.933375,25.821094,0.642406,0.687141,0.790641,0.661539,0.810469],
                "avg"=>&[2.001953,0.530762,9.799805,46.224609,0.483906,97.734375,0.467281,15.457617,0.32168,0.344105,0.652996,0.577436,0.728711],
                "min%"=>&[-99.804688,-87.695312,-82.226562,-79.882812,-99.804688,-55.664062,-99.804688,-63.085937,-99.804688,-99.804688,-22.851562,-14.648438,-11.132812],
                "MAX%"=>&[100.0,100.0,100.0,100.0,58.984375,100.0,53.515625,87.109375,31.640625,25.390625,18.359375,14.453125,11.328125],
                "range%"=>&[199.804688,187.695312,182.226562,179.882812,158.789062,155.664062,153.320312,150.195312,131.445313,125.195312,41.210937,29.101562,22.460938]).unwrap();
    }
    #[test]
    fn fn_test(){
        let filename = "tests/xor.jsm";
        let config = MarginConfig::new();
        let df = get_margines(
            filename,
            vec!["P(3|X5|X34)","P(3|X5|X44)","P(3|X5|X54)","P(3|X5|X71)"],
            config,
        );
        println!("{:?}",df);
        dataframe_to_json(df)
    }
}
