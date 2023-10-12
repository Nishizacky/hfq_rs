pub mod simulation;
pub mod get_margines;
pub use simulation::*;
pub use get_margines::*;

/*
構想：modで一つのnetlist関していろいろ操作ができるように関数間でデータを共有できるようにしておきたい
共有させておきたい関数: simulation, plot, critical_margine
*/

#[cfg(test)]
mod tests {
    use crate::{simulation, MarginConfig, get_switch_timing};
    #[test]
    fn simulation_test() {
        let filename = "/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm";
        // --snip--
        // let contents = fs::read_to_string(&filename).unwrap();
        // テキストは\n{}です
        // println!("With text:\n{}", contents);
        // print!("{:?}",simulation(&contents));
        print!("{:?}",simulation(filename,true));
    }
    #[test]
    fn function_test(){
        let ref_data_start_time =100e-12;
        let ref_data_end_time = 450e-12;
        let pulse_error = 150e-12;
        let ref_element_name = String::from("P(29|X_XOR,44|X_XOR)");
        let config = MarginConfig {ref_data_start_time,ref_data_end_time,pulse_error,ref_element_name};
        let dataframe = simulation("/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm",true).unwrap();
        let result = get_switch_timing(&config,&dataframe, true);
        println!("{:?}",result);
    }
    #[test]
    fn val_change_test(){
        let filename = "quicktest.jsm";

    }
}
