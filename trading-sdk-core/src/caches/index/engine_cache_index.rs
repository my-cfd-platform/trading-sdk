use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use stopwatch::Stopwatch;

use crate::{EngineCacheQueryBuilder, TradingCacheIndexGenerator};

#[derive(Debug)]
pub struct TradingCacheIndex {
    pub base: HashMap<String, HashSet<Arc<String>>>,
    pub quote: HashMap<String, HashSet<Arc<String>>>,
    pub collateral: HashMap<String, HashSet<Arc<String>>>,
    pub client_identification: HashMap<String, HashSet<Arc<String>>>,
    pub account_identification: HashMap<String, HashSet<Arc<String>>>,
}

impl TradingCacheIndex {
    pub fn new() -> Self {
        Self {
            base: HashMap::new(),
            quote: HashMap::new(),
            collateral: HashMap::new(),
            client_identification: HashMap::new(),
            account_identification: HashMap::new(),
        }
    }

    pub fn add_index(&mut self, target: &impl TradingCacheIndexGenerator) {
        let id = Arc::new(target.get_id());
        let base = target.get_base();
        let quote = target.get_quote();
        let collateral = target.get_collateral();
        let client = target.get_client_identification_index();
        let account = target.get_account_identification_index();

        Self::add_single_index(&mut self.base, id.clone(), base);
        Self::add_single_index(&mut self.quote, id.clone(), quote);
        Self::add_single_index(&mut self.collateral, id.clone(), collateral);
        Self::add_single_index(&mut self.client_identification, id.clone(), client);
        Self::add_single_index(&mut self.account_identification, id.clone(), account);
    }

    pub fn remove_index(&mut self, indx: &str) {
        let mut sw = Stopwatch::new();
        let id = Arc::new(indx.to_string());
        Self::remove_index_single(&mut self.base, id.clone());
        Self::remove_index_single(&mut self.quote, id.clone());
        Self::remove_index_single(&mut self.collateral, id.clone());
        Self::remove_index_single(&mut self.client_identification, id.clone());
        Self::remove_index_single(&mut self.account_identification, id.clone());
        sw.stop();

        println!("Remove took: {:?} nanos", sw.elapsed().as_nanos());
    }

    fn remove_index_single(indexses: &mut HashMap<String, HashSet<Arc<String>>>, id: Arc<String>) {
        for (_, set) in indexses {
            set.remove(&id);
        }
    }

    pub fn query(&self, query: &EngineCacheQueryBuilder) -> HashSet<Arc<String>> {
        let mut sets = vec![];

        if let Some(base) = &query.base {
            if let Some(base_ids) = self.base.get(base) {
                sets.push(base_ids.clone());
            }
        }

        if let Some(quote) = &query.quote {
            if let Some(quote_ids) = self.quote.get(quote) {
                sets.push(quote_ids.clone());
            }
        }

        if let Some(collateral) = &query.collateral {
            if let Some(collateral_ids) = self.collateral.get(collateral) {
                sets.push(collateral_ids.clone());
            }
        }

        if let Some(account) = &query.account {
            if let Some(account_ids) = self.account_identification.get(account) {
                sets.push(account_ids.clone());
            }
        }

        if let Some(client) = &query.client {
            if let Some(client_ids) = self.client_identification.get(client) {
                sets.push(client_ids.clone());
            }
        }

        // println!("Self: {:#?}", self);
        // println!("Query: {:#?}", query);

        let mut to_search = sets
            .into_iter()
            .filter_map(|x| {
                if x.len() > 0 {
                    return Some(x);
                };

                return None;
            })
            .collect::<Vec<_>>();

        if to_search.len() == 0 {
            return HashSet::default();
        }

        if to_search.len() == 0 {
            return to_search[0].clone();
        }

        let mut result = to_search[0].clone();

        for set in to_search.iter_mut().skip(1) {
            result = result.intersection(set).cloned().collect();
        }
        return result;
    }

    fn add_single_index(
        indexses: &mut HashMap<String, HashSet<Arc<String>>>,
        id: Arc<String>,
        value: Option<String>,
    ) {
        if let Some(value) = value {
            let set = indexses.entry(value).or_insert_with(HashSet::new);
            set.insert(id);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{EngineCacheQueryBuilder, TradingCacheIndex, TradingCacheIndexGenerator};

    struct TestIndexStruct {
        pub id: String,
        pub base: String,
        pub quote: String,
        pub collateral: String,
        pub client_ident: String,
        pub account_ident: String,
    }

    impl TestIndexStruct {
        pub fn new(
            id: &str,
            base: &str,
            quote: &str,
            collateral: &str,
            client_ident: &str,
            account_ident: &str,
        ) -> Self {
            Self {
                id: id.to_string(),
                base: base.to_string(),
                quote: quote.to_string(),
                collateral: collateral.to_string(),
                client_ident: client_ident.to_string(),
                account_ident: account_ident.to_string(),
            }
        }
    }

    impl TradingCacheIndexGenerator for TestIndexStruct {
        fn get_id(&self) -> String {
            self.id.clone()
        }

        fn get_base(&self) -> Option<String> {
            Some(self.base.clone())
        }

        fn get_quote(&self) -> Option<String> {
            Some(self.quote.clone())
        }

        fn get_collateral(&self) -> Option<String> {
            Some(self.collateral.clone())
        }

        fn get_client_identification_index(&self) -> Option<String> {
            Some(self.client_ident.clone())
        }

        fn get_account_identification_index(&self) -> Option<String> {
            Some(self.account_ident.clone())
        }
    }

    #[test]
    fn test_search_by_client_ident_single() {
        let mut cache = TradingCacheIndex::new();

        let mut query = EngineCacheQueryBuilder::new();
        query.with_client("client_ident");

        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        let result = cache.query(&query);

        assert_eq!(result.len(), 3);

        assert!(result.contains(&"test_id1".to_string()));
        assert!(result.contains(&"test_id2".to_string()));
        assert!(result.contains(&"test_id3".to_string()));
    }

    #[test]
    fn test_search_by_client_ident_few() {
        let mut cache = TradingCacheIndex::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident",
        ));

        let mut query1 = EngineCacheQueryBuilder::new();
        query1.with_client("client_ident");

        let mut query2 = EngineCacheQueryBuilder::new();
        query2.with_client("client_ident2");

        let mut query3 = EngineCacheQueryBuilder::new();
        query3.with_client("client_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 2);
        assert_eq!(result3.len(), 1);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result1.contains(&"test_id3".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));

        assert!(result2.contains(&"test_id2".to_string()));
        assert!(result2.contains(&"test_id5".to_string()));

        assert!(result3.contains(&"test_id6".to_string()));
    }

    #[test]
    fn test_search_by_client_account_single() {
        let mut cache = TradingCacheIndex::new();

        let mut query = EngineCacheQueryBuilder::new();
        query.with_account("account_ident");

        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        let result = cache.query(&query);

        assert_eq!(result.len(), 3);

        assert!(result.contains(&"test_id1".to_string()));
        assert!(result.contains(&"test_id2".to_string()));
        assert!(result.contains(&"test_id3".to_string()));
    }

    #[test]
    fn test_search_by_client_account_few() {
        let mut cache = TradingCacheIndex::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident2",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident2",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident3",
        ));

        let mut query1 = EngineCacheQueryBuilder::new();
        query1.with_account("account_ident");

        let mut query2 = EngineCacheQueryBuilder::new();
        query2.with_account("account_ident2");

        let mut query3 = EngineCacheQueryBuilder::new();
        query3.with_account("account_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 2);
        assert_eq!(result3.len(), 1);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result1.contains(&"test_id3".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));

        assert!(result2.contains(&"test_id2".to_string()));
        assert!(result2.contains(&"test_id5".to_string()));

        assert!(result3.contains(&"test_id6".to_string()));
    }

    #[test]
    fn test_search_by_account_and_base() {
        let mut cache = TradingCacheIndex::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base1",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident1",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base3",
            "quote",
            "collateral",
            "client_ident",
            "account_ident3",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base3",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident3",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base3",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident3",
        ));

        let mut query1 = EngineCacheQueryBuilder::new();
        query1.with_base("base3");
        query1.with_account("account_ident3");

        let mut query2 = EngineCacheQueryBuilder::new();
        query2.with_base("base2");
        query2.with_account("account_ident2");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 0);

        assert!(result1.contains(&"test_id6".to_string()));
        assert!(result1.contains(&"test_id5".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));
    }

    #[test]
    fn test_search_by_all() {
        let mut cache = TradingCacheIndex::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote3",
            "collateral",
            "client_ident3",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base1",
            "quote",
            "collateral0",
            "client_ident2",
            "account_ident1",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        //1 - test_id1
        let mut query1 = EngineCacheQueryBuilder::new();
        query1.with_base("base");
        query1.with_quote("quote3");
        query1.with_account("account_ident");

        //2 - test_id1, test_id3
        let mut query2 = EngineCacheQueryBuilder::new();
        query2.with_collateral("collateral");
        query2.with_base("base");
        query2.with_account("account_ident");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);

        assert_eq!(result1.len(), 1);
        assert_eq!(result2.len(), 2);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result2.contains(&"test_id1".to_string()));
        assert!(result2.contains(&"test_id3".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut cache = TradingCacheIndex::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident",
        ));

        let mut query1 = EngineCacheQueryBuilder::new();
        query1.with_client("client_ident");

        let mut query2 = EngineCacheQueryBuilder::new();
        query2.with_client("client_ident2");

        let mut query3 = EngineCacheQueryBuilder::new();
        query3.with_client("client_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 2);
        assert_eq!(result3.len(), 1);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result1.contains(&"test_id3".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));

        assert!(result2.contains(&"test_id2".to_string()));
        assert!(result2.contains(&"test_id5".to_string()));

        assert!(result3.contains(&"test_id6".to_string()));

        cache.remove_index("test_id1");
        cache.remove_index("test_id3");
        cache.remove_index("test_id5");
        cache.remove_index("test_id6");

        let mut query1 = EngineCacheQueryBuilder::new();
        query1.with_client("client_ident");

        let mut query2 = EngineCacheQueryBuilder::new();
        query2.with_client("client_ident2");

        let mut query3 = EngineCacheQueryBuilder::new();
        query3.with_client("client_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 1);
        assert_eq!(result2.len(), 1);
        assert_eq!(result3.len(), 0);

        assert!(result1.contains(&"test_id4".to_string()));
        assert!(result2.contains(&"test_id2".to_string()));
    }
}
