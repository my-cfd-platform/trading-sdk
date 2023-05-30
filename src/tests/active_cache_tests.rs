#![allow(unused_imports)]
use crate::{ActivePricesCache, ExecutionBidAsk, ExecutionPositionBase, ActiveExecutionPosition, PositionsStoreIndexAccessor, PositionSide};

#[derive(Clone)]
pub struct TestPositions{
    account: String,
    id: String,
    asset: String,
    base: String,
    quote: String,
    side: PositionSide,
    invest: f64,
    so_percent: f64,

}
// ExecutionPositionBase + ActiveExecutionPosition + PositionsStoreIndexAccessor
impl ExecutionPositionBase for TestPositions{
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_account_id(&self) -> &str {
        &self.account
    }

    fn get_asset_pair(&self) -> &str {
        &self.asset
    }

    fn get_side(&self) -> &crate::PositionSide {
        &self.side
    }

    fn get_invest_amount(&self) -> f64 {
        self.invest
    }

    fn get_so_percent(&self) -> f64 {
        self.so_percent
    }

    fn get_position_close_reason(&self) -> Option<crate::ExecutionClosePositionReason> {
        None
    }
}
impl ActiveExecutionPosition for TestPositions{
    fn get_profit(&self) -> f64 {
        todo!()
    }

    fn get_open_price(&self) -> f64 {
        todo!()
    }

    fn get_next_charge_settlement_fee_date(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        todo!()
    }

    fn get_last_charge_settlement_fee_date(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        todo!()
    }

    fn get_charge_settlement_fee_period_in_seconds(&self) -> Option<chrono::Duration> {
        todo!()
    }

    fn get_take_profit_in_order_profit(&self) -> Option<f64> {
        todo!()
    }

    fn get_take_profit_in_asset_price(&self) -> Option<f64> {
        todo!()
    }

    fn get_stop_loss_in_order_profit(&self) -> Option<f64> {
        todo!()
    }

    fn get_stop_loss_in_asset_price(&self) -> Option<f64> {
        todo!()
    }

    fn get_last_close_price(&self) -> f64 {
        todo!()
    }

    fn handle_bid_ask<T: ExecutionBidAsk>(&mut self, bid_ask: &T) {
        todo!()
    }
}

impl PositionsStoreIndexAccessor for TestPositions {
    fn get_account_index(&self) -> Option<String> {
        todo!()
    }

    fn get_base_coll_index(&self) -> Option<String> {
        todo!()
    }

    fn get_quote_coll_index(&self) -> Option<String> {
        todo!()
    }

    fn get_instrument_index(&self) -> Option<String> {
        todo!()
    }
}
