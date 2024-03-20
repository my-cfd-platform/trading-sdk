use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::TestEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtBidAsk {
    pub asset_pair: String,
    pub bid: f64,
    pub ask: f64,
    pub base: String,
    pub quote: String,
    pub date: DateTimeAsMicroseconds,
}

impl TestEntity for MtBidAsk {
    fn generate_test_entity() -> Self {
        Self {
            asset_pair: "BASEQUOTE".to_string(),
            bid: 25.0,
            ask: 25.0,
            base: "base".to_string(),
            quote: "quote".to_string(),
            date: DateTimeAsMicroseconds::now(),
        }
    }
}
