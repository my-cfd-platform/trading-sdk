use crate::{MtPosition, MtPositionActiveState, MtPositionSide};

pub fn update_unrealized_pl(position: &mut MtPosition<MtPositionActiveState>) {
    let is_inverted_instrument = position.base_data.quote != position.base_data.collateral;

    let side_coefficient = match position.base_data.side {
        MtPositionSide::Buy => 1.0,
        MtPositionSide::Sell => -1.0,
    };

    let volume = position.base_data.invest_amount * position.base_data.leverage;

    let investment_volume = match is_inverted_instrument {
        true => volume,
        false => volume / position.state.open_data.asset_open_price,
    };

    let price_change =
        position.state.asset_active_price - position.state.open_data.asset_open_price;

    let instrument_profit = investment_volume * price_change;

    let collateral_profit = match is_inverted_instrument {
        true => instrument_profit / position.state.asset_active_price,
        false => instrument_profit,
    };

    position.state.profit = collateral_profit * side_coefficient + position.state.swaps.total;
}

pub fn update_collateral_unrealized_pl(position: &mut MtPosition<MtPositionActiveState>) {
    let inverted_base = position
        .state
        .open_data
        .base_collateral_open_bid_ask
        .as_ref()
        .unwrap()
        .quote
        != position.base_data.collateral;
    let volume = position.base_data.invest_amount * position.base_data.leverage;

    let investment_volume = match inverted_base {
        true => volume * position.state.open_data.base_collateral_open_price,
        false => volume / position.state.open_data.base_collateral_open_price,
    };

    let price_change =
        position.state.asset_active_price - position.state.open_data.asset_open_price;

    let base_profit = investment_volume * price_change;

    //account currency profit

    let inverted_quote = position
        .state
        .quote_collateral_active_bid_ask
        .as_ref()
        .unwrap()
        .quote
        != position.base_data.collateral;

    let collateral_currency_profit = match inverted_quote {
        true => base_profit / position.state.quote_collateral_active_price,
        false => base_profit * position.state.quote_collateral_active_price,
    };

    let side_coefficient = match position.base_data.side {
        MtPositionSide::Buy => 1.0,
        MtPositionSide::Sell => -1.0,
    };

    position.state.profit = collateral_currency_profit * side_coefficient + position.state.swaps.total;
}

pub fn update_position_pl(position: &mut MtPosition<MtPositionActiveState>) {
    if position.base_data.base == position.base_data.collateral
        || position.base_data.quote == position.base_data.collateral
    {
        update_unrealized_pl(position);
    } else {
        update_collateral_unrealized_pl(position)
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{
        get_close_price, get_open_price, update_position_pl, MtBidAsk, MtPosition,
        MtPositionActiveState, MtPositionActiveStateOpenData, MtPositionBaseData, MtPositionSwaps,
    };

    #[test]
    fn calculate_pl() {
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

        let active_state = MtPositionActiveState {
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

        assert_eq!(format!("{:.2}", position.state.profit), 18.71.to_string());
    }

    #[test]
    fn calculate_pl_2() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "USDCAD".to_string(),
            bid: 1.32162,
            ask: 1.32166,
            base: "USD".to_string(),
            quote: "CAD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "USDCAD".to_string(),
            side: crate::MtPositionSide::Buy,
            invest_amount: 1000.0,
            leverage: 20.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "USD".to_string(),
            quote: "CAD".to_string(),
            tp_profit: None,
            tp_price: None,
            sl_profit: None,
            sl_price: None,
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: 1.0,
            base_collateral_open_bid_ask: None,
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "USDCAD".to_string(),
            bid: 1.34398,
            ask: 1.34402,
            base: "USD".to_string(),
            quote: "CAD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let active_state = MtPositionActiveState {
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

        assert_eq!(format!("{:.4}", position.state.profit), 332.1478.to_string());
    }


    #[test]
    fn calculate_pl_3() {
        let asset_bid_ask = MtBidAsk {
            asset_pair: "GBPCAD".to_string(),
            bid: 1.64432,
            ask: 1.64447,
            base: "GBP".to_string(),
            quote: "CAD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_collateral_bid_ask = MtBidAsk {
            asset_pair: "GBPUSD".to_string(),
            bid: 1.248,
            ask: 1.2482,
            base: "GBP".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };



        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "GBPCAD".to_string(),
            side: crate::MtPositionSide::Buy,
            invest_amount: 1000.0,
            leverage: 20.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "GBP".to_string(),
            quote: "CAD".to_string(),
            tp_profit: None,
            tp_price: None,
            sl_profit: None,
            sl_price: None,
        };

        let open_data: MtPositionActiveStateOpenData = MtPositionActiveStateOpenData {
            asset_open_price: get_open_price(&asset_bid_ask, &crate::MtPositionSide::Buy),
            asset_open_bid_ask: asset_bid_ask.clone(),
            base_collateral_open_price: get_open_price(&base_collateral_bid_ask, &crate::MtPositionSide::Buy),
            base_collateral_open_bid_ask: Some(base_collateral_bid_ask.clone()),
            open_process_id: "process".to_string(),
            open_date: DateTimeAsMicroseconds::now(),
            pending_state: None,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "GBPCAD".to_string(),
            bid: 1.62432,
            ask: 1.62447,
            base: "GBP".to_string(),
            quote: "CAD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let close_quote_collateral_bid_ask = MtBidAsk {
            asset_pair: "USDCAD".to_string(),
            bid: 1.34398,
            ask: 1.34402,
            base: "USD".to_string(),
            quote: "CAD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let active_state = MtPositionActiveState {
            open_data,
            asset_active_price: get_close_price(&close_asset_bid_ask, &base_data.side),
            asset_active_bid_ask: close_asset_bid_ask,
            quote_collateral_active_price: get_close_price(&close_quote_collateral_bid_ask, &crate::MtPositionSide::Buy),
            quote_collateral_active_bid_ask: Some(close_quote_collateral_bid_ask.clone()),
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
        };

        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);

        assert_eq!(format!("{:.4}", position.state.profit), (-240.2305).to_string());
    }
}
