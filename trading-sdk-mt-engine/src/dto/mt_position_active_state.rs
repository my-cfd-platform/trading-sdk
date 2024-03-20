use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{MtBidAsk, MtPositionPendingState, MtPositionSwaps, TestEntity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtPositionActiveStateOpenData {
    pub asset_open_price: f64,
    pub asset_open_bid_ask: MtBidAsk,
    pub base_collateral_open_price: f64,
    pub base_collateral_open_bid_ask: Option<MtBidAsk>,
    pub open_process_id: String,
    pub open_date: DateTimeAsMicroseconds,
    pub pending_state: Option<MtPositionPendingState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtPositionActiveState {
    pub open_data: MtPositionActiveStateOpenData,
    pub asset_active_price: f64,
    pub asset_active_bid_ask: MtBidAsk,
    pub quote_collateral_active_price: f64,
    pub quote_collateral_active_bid_ask: Option<MtBidAsk>,
    pub profit: f64,
    pub swaps: MtPositionSwaps,
    pub topping_up: Option<f64>,
    pub is_margin_call_hit: bool,
}

impl TestEntity for MtPositionActiveStateOpenData {
    fn generate_test_entity() -> Self {
        Self {
            asset_open_price: 10.0,
            asset_open_bid_ask: MtBidAsk::generate_test_entity(),
            base_collateral_open_price: 10.0,
            base_collateral_open_bid_ask: Some(MtBidAsk::generate_test_entity()),
            open_process_id: "open_process_id".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        }
    }
}

impl TestEntity for MtPositionActiveState {
    fn generate_test_entity() -> Self {
        Self {
            open_data: MtPositionActiveStateOpenData::generate_test_entity(),
            asset_active_price: 25.0,
            asset_active_bid_ask: MtBidAsk::generate_test_entity(),
            quote_collateral_active_price: 25.0,
            quote_collateral_active_bid_ask: Some(MtBidAsk::generate_test_entity()),
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
            topping_up: None,
            is_margin_call_hit: false,
        }
    }
}
