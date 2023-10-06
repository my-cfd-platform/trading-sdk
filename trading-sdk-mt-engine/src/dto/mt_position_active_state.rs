use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{MtBidAsk, MtPositionPendingState, MtPositionSwaps};

#[derive(Debug, Clone)]
pub struct MtPositionActiveStateOpenData {
    pub asset_open_price: f64,
    pub asset_open_bid_ask: MtBidAsk,
    pub base_collateral_open_price: f64,
    pub base_collateral_open_bid_ask: Option<MtBidAsk>,
    pub open_process_id: String,
    pub open_date: DateTimeAsMicroseconds,
    pub pending_state: Option<MtPositionPendingState>,
}

#[derive(Debug, Clone)]
pub struct MtPositionActiveState {
    pub open_data: MtPositionActiveStateOpenData,
    pub asset_active_price: f64,
    pub asset_active_bid_ask: MtBidAsk,
    pub quote_collateral_active_price: f64,
    pub quote_collateral_active_bid_ask: Option<MtBidAsk>,
    pub profit: f64,
    pub swaps: MtPositionSwaps,
}
