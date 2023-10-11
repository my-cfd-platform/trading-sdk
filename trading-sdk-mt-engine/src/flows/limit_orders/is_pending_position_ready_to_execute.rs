use crate::{get_open_price, MtBidAsk, MtPosition, MtPositionPendingState, MtPositionSide};

pub fn is_ready_to_execute_pending_position(
    position: &MtPosition<MtPositionPendingState>,
    current_bid_ask: &MtBidAsk,
) -> bool {
    let position_type = &position.state.position_type;

    match position_type {
        crate::MtPositionPendingStateType::BuyStop => {
            get_open_price(current_bid_ask, &MtPositionSide::Buy) >= position.state.desire_price
        }
        crate::MtPositionPendingStateType::BuyLimit => {
            get_open_price(current_bid_ask, &MtPositionSide::Buy) <= position.state.desire_price
        }
        crate::MtPositionPendingStateType::SellLimit => {
            get_open_price(current_bid_ask, &MtPositionSide::Sell) >= position.state.desire_price
        }
        crate::MtPositionPendingStateType::SellStop => {
            get_open_price(current_bid_ask, &MtPositionSide::Sell) <= position.state.desire_price
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{
        get_close_price, get_pending_position_type, MtBidAsk, MtPosition, MtPositionBaseData,
        MtPositionPendingState, is_ready_to_execute_pending_position, MtPositionPendingStateType,
    };

    #[test]
    fn text_buy_stop() {
        let current_price = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let desire_price = 2.0000;

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

        let state: MtPositionPendingState = MtPositionPendingState {
            desire_price,
            position_type: get_pending_position_type(
                get_close_price(&current_price, &base_data.side),
                desire_price,
                &base_data.side,
            ),
        };

        let position = MtPosition { state: state.clone(), base_data };

        let not_execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.9999,
            ask: 1.9999,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.9999,
            ask: 2.0000,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        assert_eq!(true, matches!(state.position_type, MtPositionPendingStateType::BuyStop));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &current_price));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &not_execute_bid_ask));
        assert_eq!(true, is_ready_to_execute_pending_position(&position, &execute_bid_ask));
    }

    #[test]
    fn text_buy_limit() {
        let current_price = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let desire_price = 0.9999;

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

        let state: MtPositionPendingState = MtPositionPendingState {
            desire_price,
            position_type: get_pending_position_type(
                get_close_price(&current_price, &base_data.side),
                desire_price,
                &base_data.side,
            ),
        };

        let position = MtPosition { state: state.clone(), base_data };

        let not_execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.9999,
            ask: 1.000,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.9999,
            ask: 0.9999,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        assert_eq!(true, matches!(state.position_type, MtPositionPendingStateType::BuyLimit));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &current_price));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &not_execute_bid_ask));
        assert_eq!(true, is_ready_to_execute_pending_position(&position, &execute_bid_ask));
    }


    #[test]
    fn text_sell_limit() {
        let current_price = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let desire_price = 3.0000;

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
            sl_price: None,
        };

        let state: MtPositionPendingState = MtPositionPendingState {
            desire_price,
            position_type: get_pending_position_type(
                get_close_price(&current_price, &base_data.side),
                desire_price,
                &base_data.side,
            ),
        };

        let position = MtPosition { state: state.clone(), base_data };

        let not_execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 2.9999,
            ask: 1.000,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 3.0000,
            ask: 0.9999,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        assert_eq!(true, matches!(state.position_type, MtPositionPendingStateType::SellLimit));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &current_price));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &not_execute_bid_ask));
        assert_eq!(true, is_ready_to_execute_pending_position(&position, &execute_bid_ask));
    }

    #[test]
    fn test_sell_stop() {
        let current_price = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0588,
            ask: 1.0688,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let desire_price = 1.0000;

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
            sl_price: None,
        };

        let state: MtPositionPendingState = MtPositionPendingState {
            desire_price,
            position_type: get_pending_position_type(
                get_close_price(&current_price, &base_data.side),
                desire_price,
                &base_data.side,
            ),
        };

        let position = MtPosition { state: state.clone(), base_data };

        let not_execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0001,
            ask: 1.000,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        let execute_bid_ask = MtBidAsk {
            asset_pair: "EURUSD".to_string(),
            bid: 1.0000,
            ask: 0.9999,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        };

        assert_eq!(true, matches!(state.position_type, MtPositionPendingStateType::SellStop));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &current_price));
        assert_eq!(false, is_ready_to_execute_pending_position(&position, &not_execute_bid_ask));
        assert_eq!(true, is_ready_to_execute_pending_position(&position, &execute_bid_ask));
    }
}
