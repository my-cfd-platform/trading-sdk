use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Serialize, Deserialize};

use crate::{
    get_base_collateral_open_price, get_close_price, get_open_price,
    get_quote_collateral_close_price, update_position_pl, MtBidAskCache, MtEngineError, MtPosition,
    MtPositionActiveState, MtPositionActiveStateOpenData, MtPositionBaseData,
    MtPositionPendingState, MtPositionSide, sanitize_sl_tp,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtPositionOpenCommand {
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
    pub pending_state: Option<MtPositionPendingState>,
    pub tp_profit: Option<f64>,
    pub tp_price: Option<f64>,
    pub sl_profit: Option<f64>,
    pub sl_price: Option<f64>,
}

pub fn make_active_position(
    open_command: MtPositionOpenCommand,
    prices_cache: &MtBidAskCache,
) -> Result<MtPosition<MtPositionActiveState>, MtEngineError> {
    let asset_price = prices_cache
        .get_by_id(&open_command.asset_pair)
        .ok_or(MtEngineError::NoLiquidity)?;

    let (base_collateral_open_price, base_collateral_open_bid_ask) =
        get_base_collateral_open_price(
            prices_cache,
            &open_command.base,
            &open_command.collateral,
            &open_command.side,
        )?;

    let (quote_collateral_close_price, quote_collateral_close_bid_ask) =
        get_quote_collateral_close_price(
            prices_cache,
            &open_command.quote,
            &open_command.collateral,
            &open_command.side,
        )?;

    let open_data = MtPositionActiveStateOpenData {
        asset_open_price: get_open_price(&asset_price, &open_command.side),
        asset_open_bid_ask: asset_price.as_ref().clone(),
        base_collateral_open_price,
        base_collateral_open_bid_ask,

        open_process_id: open_command.process_id.clone(),
        open_date: DateTimeAsMicroseconds::now(),
        pending_state: open_command.pending_state,
    };

    let active_state = MtPositionActiveState {
        open_data,
        asset_active_price: get_close_price(asset_price.as_ref(), &open_command.side),
        asset_active_bid_ask: asset_price.as_ref().clone(),
        quote_collateral_active_price: quote_collateral_close_price,
        quote_collateral_active_bid_ask: quote_collateral_close_bid_ask,
        profit: 0.0,
        swaps: crate::MtPositionSwaps::default(),
    };

    let mut base_data = MtPositionBaseData {
        id: open_command.id,
        trader_id: open_command.trader_id,
        account_id: open_command.account_id,
        asset_pair: open_command.asset_pair,
        side: open_command.side,
        invest_amount: open_command.invest_amount,
        leverage: open_command.leverage,
        stop_out_percent: open_command.stop_out_percent,
        create_process_id: open_command.process_id.clone(),
        crate_date: DateTimeAsMicroseconds::now(),
        last_update_process_id: open_command.process_id.clone(),
        last_update_date: DateTimeAsMicroseconds::now(),
        collateral: open_command.collateral,
        base: open_command.base,
        quote: open_command.quote,
        tp_profit: open_command.tp_profit,
        tp_price: open_command.tp_price,
        sl_profit: open_command.sl_profit,
        sl_price: open_command.sl_price,
    };

    sanitize_sl_tp(&mut base_data);

    let mut position = MtPosition {
        state: active_state,
        base_data,
    };

    update_position_pl(&mut position);

    Ok(position)
}
