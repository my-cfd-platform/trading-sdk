use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::TestEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MtPositionSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub margin_call_percent: Option<f64>,
    pub topping_up_percent: Option<f64>,
    pub metadata: Option<HashMap<String, String>>,
}

impl TestEntity for MtPositionBaseData {
    fn generate_test_entity() -> Self {
        Self {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "asset_pair".to_string(),
            side: MtPositionSide::Buy,
            invest_amount: 100.0,
            leverage: 100.0,
            stop_out_percent: 20.0,
            create_process_id: "create_process_id".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "last_update_process_id".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "collateral".to_string(),
            base: "base".to_string(),
            quote: "quote".to_string(),
            tp_profit: None,
            tp_price: None,
            sl_profit: None,
            sl_price: None,
            margin_call_percent: None,
            topping_up_percent: None,
            metadata: None,
        }
    }
}
