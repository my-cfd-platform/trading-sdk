use crate::{get_close_price, MtBidAsk, MtPosition, MtPositionActiveState};

pub fn update_active_position_rate(
    position: &mut MtPosition<MtPositionActiveState>,
    new_bid_ask: &MtBidAsk,
) {
    if position.base_data.base == new_bid_ask.base && position.base_data.quote == new_bid_ask.quote
    {
        position.state.asset_active_price = get_close_price(new_bid_ask, &position.base_data.side);
        position.state.asset_active_bid_ask = new_bid_ask.clone();
    }

    if position.base_data.quote == position.base_data.collateral {
        return;
    }

    if is_quote_collateral(
        new_bid_ask,
        &position.base_data.quote,
        &position.base_data.collateral,
    ) {
        position.state.quote_collateral_active_price =
            get_close_price(new_bid_ask, &position.base_data.side);
        position.state.quote_collateral_active_bid_ask = Some(new_bid_ask.clone());
    }
}

fn is_quote_collateral(
    bid_ask: &MtBidAsk,
    position_quote: &str,
    position_collateral: &str,
) -> bool {
    if bid_ask.base == position_quote && bid_ask.quote == position_collateral {
        return true;
    }

    if bid_ask.base == position_collateral && bid_ask.quote == position_quote {
        return true;
    }

    return false;
}
#[cfg(test)]
mod test {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{
        get_open_price, update_active_position_rate, MtBidAsk, MtPosition, MtPositionActiveState,
        MtPositionActiveStateOpenData, MtPositionBaseData, MtPositionSwaps,
    };

    #[test]
    fn rate_update_bug_467() {
        let asset_bid_ask: MtBidAsk = MtBidAsk {
            asset_pair: "USDMXN".to_string(),
            bid: 5555.0,
            ask: 5555.0,
            base: "USD".to_string(),
            quote: "MXN".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let base_data = MtPositionBaseData {
            id: "id".to_string(),
            trader_id: "trader_id".to_string(),
            account_id: "account_id".to_string(),
            asset_pair: "USDMXN".to_string(),
            side: crate::MtPositionSide::Buy,
            invest_amount: 50.0,
            leverage: 5.0,
            stop_out_percent: 30.0,
            create_process_id: "process".to_string(),
            crate_date: DateTimeAsMicroseconds::now(),
            last_update_process_id: "process".to_string(),
            last_update_date: DateTimeAsMicroseconds::now(),
            collateral: "USD".to_string(),
            base: "USD".to_string(),
            quote: "MXN".to_string(),
            tp_profit: None,
            tp_price: None,
            sl_profit: None,
            sl_price: None,
            margin_call_percent: None,
            topping_up_percent: None,
            metadata: None,
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

        let active_state = MtPositionActiveState {
            open_data,
            asset_active_price: 0.0,
            asset_active_bid_ask: asset_bid_ask.clone(),
            quote_collateral_active_price: 0.0,
            quote_collateral_active_bid_ask: None,
            profit: 0.0,
            swaps: MtPositionSwaps::default(),
            topping_up: None,
            is_margin_call_hit: false
        };

        let mut position = MtPosition {
            state: active_state,
            base_data,
        };

        let close_asset_bid_ask = MtBidAsk {
            asset_pair: "USDMXN".to_string(),
            bid: 17.919,
            ask: 17.9267,
            base: "USD".to_string(),
            quote: "MXN".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        update_active_position_rate(&mut position, &close_asset_bid_ask);
        println!("{:#?}", position);

        // assert_eq!(format!("{:.2}", position.state.profit), 18.71.to_string());
    }
}
