use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{MtBidAsk, MtPositionActiveState, MtPositionCloseReason};

#[derive(Debug, Clone)]
pub struct MtPositionClosedState {
    pub active_state: MtPositionActiveState,
    pub asset_close_price: f64,
    pub asset_close_bid_ask: MtBidAsk,
    pub close_date: DateTimeAsMicroseconds,
    pub close_process_id: String,
    pub close_reason: MtPositionCloseReason,
}
