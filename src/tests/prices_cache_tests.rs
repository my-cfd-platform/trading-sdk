#![allow(unused_imports)]
use crate::{ActivePricesCache, ExecutionBidAsk};

#[derive(Clone)]
pub struct TestBidAsk{
    asset_pair: String,
    bid: f64,
    ask: f64,
    base: String,
    quote: String,
}

impl ExecutionBidAsk for TestBidAsk {
    fn get_asset_pair(&self) -> &str {
        &self.asset_pair
    }

    fn get_bid(&self) -> f64 {
        self.bid
    }

    fn get_ask(&self) -> f64 {
        self.ask
    }

    fn get_date(&self) -> u64 {
        0
    }

    fn get_base(&self) -> &str {
        &self.base
    }

    fn get_quote(&self) -> &str {
        &self.quote
    }

    fn reverse(&self) -> Self {
        let bid = 1.0 / self.bid;
        let ask = 1.0 / self.ask;

        Self{
            asset_pair: self.asset_pair.clone(),
            bid,
            ask,
            base: self.quote.clone(),
            quote: self.base.clone(),
        }
    }
}


#[tokio::test]
async fn test_basic_cases() {
    let price = TestBidAsk {
        asset_pair: "USDCHF".to_string(),
        bid: 0.92,
        ask: 1.92,
        base: "USD".to_string(),
        quote: "CHF".to_string(),
    };

    let mut cache = ActivePricesCache::<TestBidAsk>::new();
    cache.update(price.clone());

    let base_price_id = cache.get_by_id("USDCHF");
    let base_price_base_quote = cache.get_by_currencies(&price.base, &price.quote);
    let reversed_price = cache.get_by_currencies(&price.quote, &price.base);

    assert_eq!(true, base_price_id.is_some());
    assert_eq!(true, base_price_base_quote.is_some());
    assert_eq!(true, reversed_price.is_some());

    let base_price_id = base_price_id.unwrap();
    let base_price_base_quote = base_price_base_quote.unwrap();
    let reversed_price = reversed_price.unwrap();

    assert_eq!("USDCHF".to_string(), base_price_id.get_asset_pair());
    assert_eq!("USDCHF".to_string(), base_price_base_quote.get_asset_pair());
    assert_eq!("USDCHF".to_string(), reversed_price.get_asset_pair());

    assert_eq!("0.92".to_string(), format!("{:.2}", base_price_id.get_bid()));
    assert_eq!("0.92".to_string(), format!("{:.2}", base_price_base_quote.get_bid()));
    assert_eq!("1.08696".to_string(), format!("{:.5}", reversed_price.get_bid()));

    assert_eq!("1.92".to_string(), format!("{:.2}",base_price_id.get_ask()));
    assert_eq!("1.92".to_string(), format!("{:.2}",base_price_base_quote.get_ask()));
    assert_eq!("0.52083".to_string(), format!("{:.5}",reversed_price.get_ask()));

    assert_eq!("USD", base_price_id.get_base());
    assert_eq!("USD", base_price_base_quote.get_base());
    assert_eq!("CHF", reversed_price.get_base());

    assert_eq!("CHF", base_price_id.get_quote());
    assert_eq!("CHF", base_price_base_quote.get_quote());
    assert_eq!("USD", reversed_price.get_quote());

}
