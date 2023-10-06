use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone)]
pub enum MtPositionSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct MtPositionBaseData {
    pub id: String,
    pub trader_id: String,
    pub account_id: String,
    pub asset_pair: String,
    pub side: MtPositionSide,
    pub invest_amount: f64,
    pub leverage: f64,
    pub stop_out_percent: f64,
    pub create_process_id: String,
    pub crate_date: DateTimeAsMicroseconds,
    pub last_update_process_id: String,
    pub last_update_date: DateTimeAsMicroseconds,
    pub collateral: String,
    pub base: String,
    pub quote: String,
    pub tp_profit: Option<f64>,
    pub tp_price: Option<f64>,
    pub sl_profit: Option<f64>,
    pub sl_price: Option<f64>,
}
