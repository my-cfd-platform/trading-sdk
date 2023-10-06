use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone)]
pub struct MtBidAsk {
    pub asset_pair: String,
    pub bid: f64,
    pub ask: f64,
    pub base: String,
    pub quote: String,
    pub date: DateTimeAsMicroseconds,
}
