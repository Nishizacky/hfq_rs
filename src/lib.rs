pub mod modules;

/*
構想：modで一つのnetlist関していろいろ操作ができるように関数間でデータを共有できるようにしておきたい
共有させておきたい関数: simulation, plot, critical_margine
*/

#[cfg(test)]
mod tests {
    use crate::modules::*;
    #[test]
    fn simulation_test() {
        let filename = "/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm";
        print!("{:?}", simulation(filename));
    }
    #[test]
    fn margine_check_test(){
        let filename = "/home/nishizaki/hfq_rs/part3.jsm";
    let config = MarginConfig::new();
    println!(
        "{:?}",
        get_margines(filename, vec!["P(49|X_sink,48|X_sink)"], config, true, 8)
    );
    }
}
