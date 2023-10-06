use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{MtPosition, MtPositionActiveState, MtPositionCloseReason, MtPositionClosedState};

pub fn convert_position_to_closed(
    position: MtPosition<MtPositionActiveState>,
    close_reason: MtPositionCloseReason,
    process_id: String,
) -> MtPosition<MtPositionClosedState> {
    let state = MtPositionClosedState {
        asset_close_price: position.state.asset_active_price.clone(),
        asset_close_bid_ask: position.state.asset_active_bid_ask.clone(),
        active_state: position.state.clone(),
        close_date: DateTimeAsMicroseconds::now(),
        close_process_id: process_id,
        close_reason,
    };

    return MtPosition {
        state,
        base_data: position.base_data,
    };
}
