use crate::{get_open_price, MtBidAsk, MtPosition, MtPositionPendingState, MtPositionSide};

pub fn is_ready_to_execute_pending_position(
    position: &MtPosition<MtPositionPendingState>,
    current_bid_ask: &MtBidAsk,
) -> bool {
    let position_type = &position.state.position_type;

    match position_type {
        crate::MtPositionPendingStateType::BuyStop => {
            get_open_price(current_bid_ask, &MtPositionSide::Buy) < position.state.desire_price
        }
        crate::MtPositionPendingStateType::BuyLimit => {
            -get_open_price(current_bid_ask, &MtPositionSide::Buy) < position.state.desire_price
        }
        crate::MtPositionPendingStateType::SellLimit => {
            get_open_price(current_bid_ask, &MtPositionSide::Sell) < position.state.desire_price
        }
        crate::MtPositionPendingStateType::SellStop => {
            -get_open_price(current_bid_ask, &MtPositionSide::Sell) < position.state.desire_price
        }
    }
}
