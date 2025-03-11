pub mod get_margines;
pub mod get_margines_util;
pub mod simulation;

pub use get_margines::*;
pub use get_margines_util::*;
pub use simulation::*;

pub const PI_RATIO: f64 = 0.9;
#[derive(Clone, Default)]
pub struct MarginConfig {
    //素子名だけで組んでくれるマクロを作成しておくこと。
    // ここではシミュレーションの中で検査する時間帯や、誤差をどれだけ許容するかを指定します。
    pub ref_data_start_time: f64, //最初に回路の値がどうなっているのか初期状態を取得する必要があります。これはその参考値を取得する開始時間です
    pub ref_data_end_time: f64,   //初期状態の参考値を取得する終了時間です。
    pub pulse_error: f64, //処理対象の回路についてスイッチするタイミングの誤差を指定するものです。緩すぎると明らかな異常状態を見逃すリスクが上がります。
    pub flux_type: FlaxType,
}
#[derive(Default,Clone)]
pub enum FlaxType {
    #[default]
    HFQ,
    SQF
}
impl MarginConfig {
    pub fn new() -> Self {
        Self {
            ref_data_start_time: 200e-12,
            ref_data_end_time: 450e-12,
            pulse_error: 150e-12,
            flux_type: FlaxType::HFQ,
        }
    }
}
