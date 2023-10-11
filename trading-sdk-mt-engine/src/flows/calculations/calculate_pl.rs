use crate::{MtPosition, MtPositionActiveState, MtPositionSide};

pub fn update_position_pl(position: &mut MtPosition<MtPositionActiveState>) {
    let volume = position.base_data.invest_amount * position.base_data.leverage;

    let collateral_amount = match &position.state.open_data.base_collateral_open_bid_ask {
        Some(collateral_open_bid_ask) => {
            if position.base_data.collateral != collateral_open_bid_ask.quote {
                volume * position.state.open_data.base_collateral_open_price
            } else {
                volume / position.state.open_data.base_collateral_open_price
            }
        }
        None => volume,
    };

    let price_change =
        position.state.asset_active_price - position.state.open_data.asset_open_price;

    let pl = match &position.state.quote_collateral_active_bid_ask {
        Some(quote_collateral_bid_ask) => {
            if position.base_data.quote != quote_collateral_bid_ask.quote {
                collateral_amount * price_change * position.state.quote_collateral_active_price
            } else {
                collateral_amount * price_change / position.state.quote_collateral_active_price
            }
        }
        None => collateral_amount * price_change,
    };

    let pl = match position.base_data.side {
        MtPositionSide::Buy => pl,
        MtPositionSide::Sell => -pl,
    };

    position.state.profit = pl;
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{
        get_close_price, get_open_price, update_position_pl, MtBidAsk, MtPositionActiveState,
        MtPositionActiveStateOpenData, MtPositionBaseData, MtPositionSwaps, MtPosition,
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

        let mut position = MtPosition{
            state: active_state,
            base_data,
        };

        update_position_pl(&mut position);

        assert_eq!(format!("{:.2}", position.state.profit), 18.71.to_string());
    }
}
