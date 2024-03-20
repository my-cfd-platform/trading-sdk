mod mt_bid_ask_cache;
pub use mt_bid_ask_cache::*;

use trading_sdk_core::PositionsCache;

use crate::{MtPosition, MtPositionActiveState, MtPositionPendingState};

pub struct ActivePositionsCache(pub PositionsCache<MtPosition<MtPositionActiveState>>);

impl ActivePositionsCache {
    pub fn new() -> Self {
        Self(PositionsCache::new("active_positions".to_string()))
    }
}

pub struct PendingPositionsCache(pub PositionsCache<MtPosition<MtPositionPendingState>>);

impl PendingPositionsCache {
    pub fn new() -> Self {
        Self(PositionsCache::new("pending_positions".to_string()))
    }
}
