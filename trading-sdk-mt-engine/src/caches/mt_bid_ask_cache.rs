use std::{collections::HashMap, sync::Arc};

use crate::MtBidAsk;

#[derive(Debug, Clone)]
pub struct MtBidAskCache {
    prices: HashMap<String, Arc<MtBidAsk>>,
    base_quote_index: HashMap<String, HashMap<String, Arc<MtBidAsk>>>,
    quote_base_index: HashMap<String, HashMap<String, Arc<MtBidAsk>>>,
}

impl FromIterator<MtBidAsk> for MtBidAskCache {
    fn from_iter<T: IntoIterator<Item = MtBidAsk>>(iter: T) -> Self {
        let mut prices = HashMap::new();
        let mut base_quote_index = HashMap::new();
        let mut quote_base_index = HashMap::new();

        for bid_ask in iter {
            let bid_ask = Arc::new(bid_ask);
            prices.insert(bid_ask.asset_pair.clone(), bid_ask.clone());

            let base_quote = base_quote_index
                .entry(bid_ask.base.clone())
                .or_insert_with(HashMap::new);
            base_quote.insert(bid_ask.quote.clone(), bid_ask.clone());

            let quote_base = quote_base_index
                .entry(bid_ask.quote.clone())
                .or_insert_with(HashMap::new);
            quote_base.insert(bid_ask.base.clone(), bid_ask.clone());
        }

        Self {
            prices,
            base_quote_index,
            quote_base_index,
        }
    }
}

impl MtBidAskCache {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
            base_quote_index: HashMap::new(),
            quote_base_index: HashMap::new(),
        }
    }

    pub fn handle_new(&mut self, bid_ask: MtBidAsk) {
        let bid_ask = Arc::new(bid_ask);
        self.prices
            .insert(bid_ask.asset_pair.clone(), bid_ask.clone());

        let base_quote = self
            .base_quote_index
            .entry(bid_ask.base.clone())
            .or_insert_with(HashMap::new);
        base_quote.insert(bid_ask.quote.clone(), bid_ask.clone());

        let quote_base = self
            .quote_base_index
            .entry(bid_ask.quote.clone())
            .or_insert_with(HashMap::new);
        quote_base.insert(bid_ask.base.clone(), bid_ask.clone());
    }

    pub fn get_by_id(&self, id: &str) -> Option<Arc<MtBidAsk>> {
        self.prices.get(id).cloned()
    }

    pub fn get_base_quote(&self, base: &str, quote: &str) -> Option<Arc<MtBidAsk>> {
        self.base_quote_index
            .get(base)
            .and_then(|x| x.get(quote))
            .cloned()
    }

    pub fn get_quote_base(&self, quote: &str, base: &str) -> Option<Arc<MtBidAsk>> {
        self.quote_base_index
            .get(quote)
            .and_then(|x| x.get(base))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{MtBidAsk, MtBidAskCache};

    #[test]
    fn test_cache_from_iter() {
        let data = vec![
            MtBidAsk {
                asset_pair: "BTCUSD".to_string(),
                bid: 25555.0,
                ask: 26666.0,
                base: "BTC".to_string(),
                quote: "USD".to_string(),
                date: DateTimeAsMicroseconds::now(),
            },
            MtBidAsk {
                asset_pair: "ETHUSD".to_string(),
                bid: 2555.0,
                ask: 2666.0,
                base: "ETH".to_string(),
                quote: "USD".to_string(),
                date: DateTimeAsMicroseconds::now(),
            },
        ];

        let cache = MtBidAskCache::from_iter(data.into_iter());

        let price = cache.get_by_id("BTCUSD").unwrap();
        let base_quote = cache.get_base_quote("BTC", "USD").unwrap();
        let quote_base = cache.get_quote_base("USD", "BTC").unwrap();

        assert_eq!(price.asset_pair, "BTCUSD");
        assert_eq!(base_quote.asset_pair, "BTCUSD");
        assert_eq!(quote_base.asset_pair, "BTCUSD");

        assert_eq!(price.bid, 25555.0);
        assert_eq!(base_quote.bid, 25555.0);
        assert_eq!(quote_base.bid, 25555.0);

        assert_eq!(price.ask, 26666.0);
        assert_eq!(base_quote.ask, 26666.0);
        assert_eq!(quote_base.ask, 26666.0);

        assert_eq!(price.base, "BTC");
        assert_eq!(base_quote.base, "BTC");
        assert_eq!(quote_base.base, "BTC");

        assert_eq!(price.quote, "USD");
        assert_eq!(base_quote.quote, "USD");
        assert_eq!(quote_base.quote, "USD");

        let price = cache.get_by_id("ETHUSD").unwrap();
        let base_quote = cache.get_base_quote("ETH", "USD").unwrap();
        let quote_base = cache.get_quote_base("USD", "ETH").unwrap();

        assert_eq!(price.asset_pair, "ETHUSD");
        assert_eq!(base_quote.asset_pair, "ETHUSD");
        assert_eq!(quote_base.asset_pair, "ETHUSD");

        assert_eq!(price.bid, 2555.0);
        assert_eq!(base_quote.bid, 2555.0);
        assert_eq!(quote_base.bid, 2555.0);

        assert_eq!(price.ask, 2666.0);
        assert_eq!(base_quote.ask, 2666.0);
        assert_eq!(quote_base.ask, 2666.0);

        assert_eq!(price.base, "ETH");
        assert_eq!(base_quote.base, "ETH");
        assert_eq!(quote_base.base, "ETH");

        assert_eq!(price.quote, "USD");
        assert_eq!(base_quote.quote, "USD");
        assert_eq!(quote_base.quote, "USD");
    }

    #[test]
    fn test_cache_handle_update() {
        let mut cache = MtBidAskCache::new();

        cache.handle_new(MtBidAsk {
            asset_pair: "BTCUSD".to_string(),
            bid: 25555.0,
            ask: 26666.0,
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        });

        cache.handle_new(MtBidAsk {
            asset_pair: "ETHUSD".to_string(),
            bid: 2555.0,
            ask: 2666.0,
            base: "ETH".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        });

        let price = cache.get_by_id("BTCUSD").unwrap();
        let base_quote = cache.get_base_quote("BTC", "USD").unwrap();
        let quote_base = cache.get_quote_base("USD", "BTC").unwrap();

        assert_eq!(price.asset_pair, "BTCUSD");
        assert_eq!(base_quote.asset_pair, "BTCUSD");
        assert_eq!(quote_base.asset_pair, "BTCUSD");

        assert_eq!(price.bid, 25555.0);
        assert_eq!(base_quote.bid, 25555.0);
        assert_eq!(quote_base.bid, 25555.0);

        assert_eq!(price.ask, 26666.0);
        assert_eq!(base_quote.ask, 26666.0);
        assert_eq!(quote_base.ask, 26666.0);

        assert_eq!(price.base, "BTC");
        assert_eq!(base_quote.base, "BTC");
        assert_eq!(quote_base.base, "BTC");

        assert_eq!(price.quote, "USD");
        assert_eq!(base_quote.quote, "USD");
        assert_eq!(quote_base.quote, "USD");

        let price = cache.get_by_id("ETHUSD").unwrap();
        let base_quote = cache.get_base_quote("ETH", "USD").unwrap();
        let quote_base = cache.get_quote_base("USD", "ETH").unwrap();

        assert_eq!(price.asset_pair, "ETHUSD");
        assert_eq!(base_quote.asset_pair, "ETHUSD");
        assert_eq!(quote_base.asset_pair, "ETHUSD");

        assert_eq!(price.bid, 2555.0);
        assert_eq!(base_quote.bid, 2555.0);
        assert_eq!(quote_base.bid, 2555.0);

        assert_eq!(price.ask, 2666.0);
        assert_eq!(base_quote.ask, 2666.0);
        assert_eq!(quote_base.ask, 2666.0);

        assert_eq!(price.base, "ETH");
        assert_eq!(base_quote.base, "ETH");
        assert_eq!(quote_base.base, "ETH");

        assert_eq!(price.quote, "USD");
        assert_eq!(base_quote.quote, "USD");
        assert_eq!(quote_base.quote, "USD");

        cache.handle_new(MtBidAsk {
            asset_pair: "ETHUSD".to_string(),
            bid: 3555.0,
            ask: 3666.0,
            base: "ETH".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::now(),
        });

        let price = cache.get_by_id("ETHUSD").unwrap();
        let base_quote = cache.get_base_quote("ETH", "USD").unwrap();
        let quote_base = cache.get_quote_base("USD", "ETH").unwrap();

        assert_eq!(price.asset_pair, "ETHUSD");
        assert_eq!(base_quote.asset_pair, "ETHUSD");
        assert_eq!(quote_base.asset_pair, "ETHUSD");

        assert_eq!(price.bid, 3555.0);
        assert_eq!(base_quote.bid, 3555.0);
        assert_eq!(quote_base.bid, 3555.0);

        assert_eq!(price.ask, 3666.0);
        assert_eq!(base_quote.ask, 3666.0);
        assert_eq!(quote_base.ask, 3666.0);

        assert_eq!(price.base, "ETH");
        assert_eq!(base_quote.base, "ETH");
        assert_eq!(quote_base.base, "ETH");

        assert_eq!(price.quote, "USD");
        assert_eq!(base_quote.quote, "USD");
        assert_eq!(quote_base.quote, "USD");
    }
}
