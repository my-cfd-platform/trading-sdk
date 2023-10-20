use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    get_open_price, get_pending_position_type, sanitize_sl_tp, MtBidAskCache, MtEngineError,
    MtPosition, MtPositionBaseData, MtPositionPendingState, MtPositionSide,
};

pub struct MtPositionOpenPendingCommand {
    pub id: String,
    pub trader_id: String,
    pub account_id: String,
    pub side: MtPositionSide,
    pub asset_pair: String,
    pub base: String,
    pub quote: String,
    pub collateral: String,
    pub invest_amount: f64,
    pub leverage: f64,
    pub stop_out_percent: f64,
    pub process_id: String,
    pub tp_profit: Option<f64>,
    pub tp_price: Option<f64>,
    pub sl_profit: Option<f64>,
    pub sl_price: Option<f64>,
    pub desired_open_price: f64,
}

pub fn create_pending_position(
    command: MtPositionOpenPendingCommand,
    prices_cache: &MtBidAskCache,
) -> Result<MtPosition<MtPositionPendingState>, MtEngineError> {
    let asset_price = prices_cache
        .get_by_id(&command.asset_pair)
        .ok_or(MtEngineError::NoLiquidity)?;

    let current_price = get_open_price(asset_price.as_ref(), &command.side);

    let position_type =
        get_pending_position_type(current_price, command.desired_open_price, &command.side);

    let state = MtPositionPendingState {
        desire_price: command.desired_open_price,
        position_type,
    };

    let mut base_data = MtPositionBaseData {
        id: command.id,
        trader_id: command.trader_id,
        account_id: command.account_id,
        asset_pair: command.asset_pair,
        side: command.side,
        invest_amount: command.invest_amount,
        leverage: command.leverage,
        stop_out_percent: command.stop_out_percent,
        create_process_id: command.process_id.clone(),
        crate_date: DateTimeAsMicroseconds::now(),
        last_update_process_id: command.process_id.clone(),
        last_update_date: DateTimeAsMicroseconds::now(),
        collateral: command.collateral,
        base: command.base,
        quote: command.quote,
        tp_profit: command.tp_profit,
        tp_price: command.tp_price,
        sl_profit: command.sl_profit,
        sl_price: command.sl_price,
    };

    sanitize_sl_tp(&mut base_data);

    return Ok(MtPosition {
        state,
        base_data: base_data,
    });
}
