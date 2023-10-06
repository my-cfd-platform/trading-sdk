use crate::{MtBidAsk, MtBidAskCache, MtPositionSide, MtPositionPendingStateType, MtEngineError};

pub fn get_open_price(bid_ask: &MtBidAsk, side: &MtPositionSide) -> f64 {
    match side {
        MtPositionSide::Buy => bid_ask.ask,
        MtPositionSide::Sell => bid_ask.bid,
    }
}

pub fn get_close_price(bid_ask: &MtBidAsk, side: &MtPositionSide) -> f64 {
    match side {
        MtPositionSide::Buy => bid_ask.bid,
        MtPositionSide::Sell => bid_ask.ask,
    }
}

pub fn get_pending_position_type(
    current_price: f64,
    desire_price: f64,
    side: &MtPositionSide,
) -> MtPositionPendingStateType {
    match side {
        MtPositionSide::Buy => {
            if desire_price > current_price {
                MtPositionPendingStateType::BuyStop
            } else {
                MtPositionPendingStateType::BuyLimit
            }
        }
        MtPositionSide::Sell => {
            if desire_price > current_price {
                MtPositionPendingStateType::SellLimit
            } else {
                MtPositionPendingStateType::SellStop
            }
        }
    }
}

pub fn get_any_price_by_tickers(
    prices_cache: &MtBidAskCache,
    ticker1: &str,
    ticker2: &str,
) -> Option<MtBidAsk> {
    if let Some(price) = prices_cache.get_base_quote(ticker1, ticker2) {
        return Some(price.as_ref().clone());
    }

    if let Some(price) = prices_cache.get_quote_base(ticker1, ticker2) {
        return Some(price.as_ref().clone());
    }

    return None;
}

pub fn get_base_collateral_open_price(
    prices_cache: &MtBidAskCache,
    base: &str,
    collateral: &str,
    side: &MtPositionSide,
) -> Result<(f64, Option<MtBidAsk>), MtEngineError> {
    if collateral == base {
        return Ok((1.0, None));
    }

    match get_any_price_by_tickers(prices_cache, collateral, base) {
        Some(src) => Ok((get_open_price(&src, side), Some(src))),
        None => Err(MtEngineError::NoLiquidity),
    }
}

pub fn get_quote_collateral_close_price(
    prices_cache: &MtBidAskCache,
    quote: &str,
    collateral: &str,
    side: &MtPositionSide,
) -> Result<(f64, Option<MtBidAsk>), MtEngineError> {
    if collateral == quote {
        return Ok((1.0, None));
    }

    match get_any_price_by_tickers(prices_cache, collateral, quote) {
        Some(src) => Ok((get_close_price(&src, side), Some(src))),
        None => Err(MtEngineError::NoLiquidity),
    }
}
