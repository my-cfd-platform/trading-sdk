use crate::{get_close_price, MtBidAsk, MtPosition, MtPositionActiveState};

pub fn update_active_position_rate(
    position: &mut MtPosition<MtPositionActiveState>,
    new_bid_ask: &MtBidAsk,
) {
    if position.base_data.base == new_bid_ask.base && position.base_data.quote == new_bid_ask.quote
    {
        position.state.asset_active_price = get_close_price(new_bid_ask, &position.base_data.side);
        position.state.asset_active_bid_ask = new_bid_ask.clone();
        return;
    }

    if let Some(_) = &position.state.quote_collateral_active_bid_ask{
        position.state.quote_collateral_active_price =
            get_close_price(new_bid_ask, &position.base_data.side);
        position.state.quote_collateral_active_bid_ask = Some(new_bid_ask.clone());
        return;
    }
}
