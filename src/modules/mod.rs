pub mod simulation;
pub mod get_margines;
pub mod get_margines_util;
pub mod plotter;

pub use simulation::*;
pub use get_margines::*;
pub use get_margines_util::*;

pub const PI_RATIO: f64 = 0.9;

pub struct MarginConfig {
    //素子名だけで組んでくれるマクロを作成しておくこと。
    // ここではシミュレーションの中で検査する時間帯や、誤差をどれだけ許容するかを指定します。
    pub ref_data_start_time: f64, //最初に回路の値がどうなっているのか初期状態を取得する必要があります。これはその参考値を取得する開始時間です
    pub ref_data_end_time: f64,   //初期状態の参考値を取得する終了時間です。
    pub pulse_error: f64, //処理対象の回路についてスイッチするタイミングの誤差を指定するものです。緩すぎると明らかな異常状態を見逃すリスクが上がります。
}
impl MarginConfig {
    pub fn new() -> MarginConfig {
        let ref_data_start_time = 200e-12;
        let ref_data_end_time = 450e-12;
        let pulse_error = 150e-12;
        MarginConfig {
            ref_data_start_time,
            ref_data_end_time,
            pulse_error,
        }
    }
}
impl Clone for MarginConfig {
    fn clone(&self) -> Self {
        MarginConfig {
            ref_data_start_time: self.ref_data_start_time,
            ref_data_end_time: self.ref_data_end_time,
            pulse_error: self.pulse_error,
        }
    }
}