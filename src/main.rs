use hfq_rs::MarginConfig;
use hfq_rs::get_margines::get_margines;
fn main(){
    let filename = "/home/nishizaki/hfq_rs/part3.jsm";
    let config = MarginConfig::new();
    println!(
        "{:?}",
        get_margines(filename, vec!["P(49|X_sink,48|X_sink)"], config, true, 8)
    );
}