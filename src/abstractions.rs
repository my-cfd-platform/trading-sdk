use chrono::{DateTime, Duration, Utc};

use crate::{ExecutionClosePositionReason, ExecutionPendingOrderType, PositionSide};

pub trait ExecutionBidAsk {
    fn get_asset_pair(&self) -> &str;
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
    fn get_date(&self) -> u64;
}

pub trait ExecutionPositionBase {
    fn get_id(&self) -> &str;
    fn get_account_id(&self) -> &str;
    fn get_asset_pair(&self) -> &str;
    fn get_side(&self) -> &PositionSide;
    fn get_volume(&self) -> f64;
    fn get_invest_amount(&self) -> f64;
    fn get_so_percent(&self) -> f64;
    fn get_position_close_reason(&self) -> Option<ExecutionClosePositionReason>;
}

pub trait ABookExecutionPosition {
    fn get_collateral_asset(&self) -> &str;
    fn get_base_coll_price(&self) -> &f64;
    fn get_base_coll_currency(&self) -> &str;
    fn get_quote_coll_price(&self) -> &f64;
    fn get_quote_coll_currency(&self) -> &str;
}

pub trait ActiveExecutionPosition {
    fn get_profit(&self) -> f64;
    fn get_open_price(&self) -> f64;
    fn get_next_charge_settlement_fee_date(&self) -> Option<DateTime<Utc>>;
    fn get_last_charge_settlement_fee_date(&self) -> Option<DateTime<Utc>>;
    fn get_charge_settlement_fee_period_in_seconds(&self) -> Option<Duration>;
    fn get_take_profit_in_order_profit(&self) -> Option<f64>;
    fn get_take_profit_in_asset_price(&self) -> Option<f64>;
    fn get_stop_loss_in_order_profit(&self) -> Option<f64>;
    fn get_stop_loss_in_asset_price(&self) -> Option<f64>;
    fn get_last_close_price(&self) -> f64;
    fn handle_bid_ask(&mut self, bid_ask: &impl ExecutionBidAsk);
}

pub trait PendingExecutionOrder {
    fn get_desired_price(&self) -> f64;
    fn get_pending_order_type(&self) -> &ExecutionPendingOrderType;
}
