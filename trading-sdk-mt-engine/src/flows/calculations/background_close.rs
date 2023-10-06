use crate::{MtPosition, MtPositionActiveState, MtPositionCloseReason};

pub fn get_close_reason(
    position: &MtPosition<MtPositionActiveState>,
) -> Option<MtPositionCloseReason> {
    if is_so_triggered(position) {
        return Some(MtPositionCloseReason::StopOut);
    }

    if is_sl_triggered(position) {
        return Some(MtPositionCloseReason::StopLoss);
    }

    if is_tp_triggered(position) {
        return Some(MtPositionCloseReason::TakeProfit);
    }

    return None;
}

fn is_so_triggered(position: &MtPosition<MtPositionActiveState>) -> bool {
    return 100.0 - calculate_position_margin_percent(position)
        >= position.base_data.stop_out_percent;
}

fn is_sl_triggered(position: &MtPosition<MtPositionActiveState>) -> bool {
    if let Some(sl) = position.base_data.sl_profit {
        return position.state.profit <= sl;
    }

    if let Some(sl) = position.base_data.sl_price {
        return match &position.base_data.side {
            crate::MtPositionSide::Buy => sl >= position.state.asset_active_price,
            crate::MtPositionSide::Sell => sl <= position.state.asset_active_price,
        };
    }

    return false;
}

fn is_tp_triggered(position: &MtPosition<MtPositionActiveState>) -> bool {
    if let Some(tp) = position.base_data.tp_profit {
        return position.state.profit >= tp;
    }

    if let Some(tp) = position.base_data.tp_price {
        return match &position.base_data.side {
            crate::MtPositionSide::Buy => tp <= position.state.asset_active_price,
            crate::MtPositionSide::Sell => tp >= position.state.asset_active_price,
        };
    }

    return false;
}

fn calculate_position_margin_percent(
    position: &MtPosition<MtPositionActiveState>,
) -> f64 {
    let margin = position.state.profit + position.base_data.invest_amount;
    return margin / position.base_data.invest_amount * 100.0;
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{
        get_close_price, get_open_price, update_position_pl, MtBidAsk, MtPosition,
        MtPositionActiveState, MtPositionActiveStateOpenData, MtPositionBaseData,
        MtPositionCloseReason, MtPositionSwaps,
    };
    #[test]
    fn test_tp_close_ok() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "EURUSD".to_string(),
            side: crate::MtPositionSide::Buy,
            invest_amount: 1000.0,
            leverage: 20.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            tp_profit: Some(18.71),
            tp_price: None,
            sl_profit: None,
            sl_price: None,
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            base_collateral_open_bid_ask: Some(asset_bid_ask.clone()),
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0698,
            ask: 1.0798,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let mut active_state = MtPositionActiveState {
            open_data,
            asset_active_price: get_close_price(&close_asset_bid_ask, &base_data.side),
            asset_active_bid_ask: close_asset_bid_ask,
            quote_collateral_active_price: 1.0,
            quote_collateral_active_bid_ask: None,
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
        };

        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);
        let cr = super::get_close_reason(&position).unwrap();

        assert_eq!(format!("{:.2}", position.state.profit), 18.71.to_string());
        assert_eq!(matches!(cr, MtPositionCloseReason::TakeProfit), true);
    }

    #[test]
    fn test_tp_profit_close() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "EURUSD".to_string(),
            side: crate::MtPositionSide::Buy,
            invest_amount: 1000.0,
            leverage: 20.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            tp_profit: Some(18.72),
            tp_price: None,
            sl_profit: None,
            sl_price: None,
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            base_collateral_open_bid_ask: Some(asset_bid_ask.clone()),
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0698,
            ask: 1.0798,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let mut active_state = MtPositionActiveState {
            open_data,
            asset_active_price: get_close_price(&close_asset_bid_ask, &base_data.side),
            asset_active_bid_ask: close_asset_bid_ask,
            quote_collateral_active_price: 1.0,
            quote_collateral_active_bid_ask: None,
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
        };

        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);
        let cr = super::get_close_reason(&position);

        assert_eq!(matches!(cr, None), true);
    }

    #[test]
    fn test_tp_price() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "EURUSD".to_string(),
            side: crate::MtPositionSide::Buy,
            invest_amount: 1000.0,
            leverage: 20.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            tp_profit: None,
            tp_price: Some(1.0697),
            sl_profit: None,
            sl_price: None,
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            base_collateral_open_bid_ask: Some(asset_bid_ask.clone()),
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0698,
            ask: 1.0798,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let mut active_state = MtPositionActiveState {
            open_data,
            asset_active_price: get_close_price(&close_asset_bid_ask, &base_data.side),
            asset_active_bid_ask: close_asset_bid_ask,
            quote_collateral_active_price: 1.0,
            quote_collateral_active_bid_ask: None,
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
        };

        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);
        let cr = super::get_close_reason(&position).unwrap();

        assert_eq!(format!("{:.2}", position.state.profit), 18.71.to_string());
        assert_eq!(matches!(cr, MtPositionCloseReason::TakeProfit), true);
    }

    #[test]
    fn test_sl_profit_close_ok() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0688,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "EURUSD".to_string(),
            side: crate::MtPositionSide::Sell,
            invest_amount: 1000.0,
            leverage: 20.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            tp_profit: None,
            tp_price: None,
            sl_profit: None,
            sl_price: Some(1.0697),
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            base_collateral_open_bid_ask: Some(asset_bid_ask.clone()),
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0698,
            ask: 1.0698,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let mut active_state = MtPositionActiveState {
            open_data,
            asset_active_price: get_close_price(&close_asset_bid_ask, &base_data.side),
            asset_active_bid_ask: close_asset_bid_ask,
            quote_collateral_active_price: 1.0,
            quote_collateral_active_bid_ask: None,
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
        };
        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);
        let cr = super::get_close_reason(&position).unwrap();

        assert_eq!(format!("{:.2}", position.state.profit), (-18.71).to_string());
        assert_eq!(matches!(cr, MtPositionCloseReason::StopLoss), true);
    }

    #[test]
    fn test_so_profit_close_ok() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0688,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "EURUSD".to_string(),
            side: crate::MtPositionSide::Sell,
            invest_amount: 100.0,
            leverage: 200.0,
            stop_out_percent: 18.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            tp_profit: None,
            tp_price: None,
            sl_profit: None,
            sl_price: None,
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            base_collateral_open_bid_ask: Some(asset_bid_ask.clone()),
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0698,
            ask: 1.0698,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let mut active_state = MtPositionActiveState {
            open_data,
            asset_active_price: get_close_price(&close_asset_bid_ask, &base_data.side),
            asset_active_bid_ask: close_asset_bid_ask,
            quote_collateral_active_price: 1.0,
            quote_collateral_active_bid_ask: None,
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
        };

        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);
        let cr = super::get_close_reason(&position).unwrap();
        assert_eq!(format!("{:.2}", position.state.profit), (-18.71).to_string());
        assert_eq!(matches!(cr, MtPositionCloseReason::StopOut), true);
    }
}
