use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::ExecutionBidAsk;

pub struct ActivePricesCache<T>
where
    T: ExecutionBidAsk,
{
    prices: HashMap<String, Arc<T>>,
}

impl<T> ActivePricesCache<T>
where
    T: ExecutionBidAsk,
{
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
        }
    }

    pub fn update(&mut self, price: T) {
        self.prices.insert(price.get_asset_pair().to_string(), Arc::new(price));
    }

    pub fn get(&self, asset: &str) -> Option<Arc<T>> {
        let price = self.prices.get(asset)?;
        return Some(price.clone());
    }

    pub fn get_all(&self) -> Vec<Arc<T>> {
        return self.prices.values().map(|price| price.clone()).collect();
    }
}
