use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    get_base_collateral_open_price, get_close_price, get_open_price,
    get_quote_collateral_close_price, MtBidAskCache, MtEngineError, MtPosition,
    MtPositionActiveState, MtPositionActiveStateOpenData, MtPositionPendingState,
};

pub fn execute_pending_position(
    pending_position: MtPosition<MtPositionPendingState>,
    prices_cache: &MtBidAskCache,
    process_id: String,
) -> Result<MtPosition<MtPositionActiveState>, MtEngineError> {
    let asset_price = prices_cache
        .get_by_id(&pending_position.base_data.asset_pair)
        .ok_or(MtEngineError::NoLiquidity)?;

    let (base_collateral_open_price, base_collateral_open_bid_ask) =
        get_base_collateral_open_price(
            prices_cache,
            &pending_position.base_data.base,
            &pending_position.base_data.collateral,
            &pending_position.base_data.side,
        )?;

    let (quote_collateral_close_price, quote_collateral_close_bid_ask) =
        get_quote_collateral_close_price(
            prices_cache,
            &pending_position.base_data.quote,
            &pending_position.base_data.collateral,
            &pending_position.base_data.side,
        )?;

    let open_date = MtPositionActiveStateOpenData {
        asset_open_price: get_open_price(asset_price.as_ref(), &pending_position.base_data.side),
        asset_open_bid_ask: asset_price.as_ref().clone(),
        base_collateral_open_price,
        base_collateral_open_bid_ask,
        open_process_id: process_id.clone(),
        open_date: DateTimeAsMicroseconds::now(),
        pending_state: Some(pending_position.state),
    };

    let active_state = MtPositionActiveState {
        open_data: open_date,
        asset_active_price: get_close_price(asset_price.as_ref(), &pending_position.base_data.side),
        asset_active_bid_ask: asset_price.as_ref().clone(),
        quote_collateral_active_price: quote_collateral_close_price,
        quote_collateral_active_bid_ask: quote_collateral_close_bid_ask,
        profit: 0.0,
        swaps: crate::MtPositionSwaps::default(),
        topping_up: None,
        is_margin_call_hit: false
    };

    return Ok(MtPosition {
        state: active_state,
        base_data: pending_position.base_data,
    });
}
