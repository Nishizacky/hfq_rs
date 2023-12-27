pub mod get_margines_util;
pub mod simulation;
pub mod get_margines;
pub use simulation::*;
pub use get_margines_util::*;

/*
構想：modで一つのnetlist関していろいろ操作ができるように関数間でデータを共有できるようにしておきたい
共有させておきたい関数: simulation, plot, critical_margine
*/

#[cfg(test)]
mod tests {
    use crate::simulation;
    #[test]
    fn simulation_test() {
        let filename = "/home/nishizaki/myHFQenv/hfq_xor/hfq_xor4share.jsm";
        // --snip--
        // let contents = fs::read_to_string(&filename).unwrap();
        // テキストは\n{}です
        // println!("With text:\n{}", contents);
        // print!("{:?}",simulation(&contents));
        print!("{:?}", simulation(filename));
    }
}
