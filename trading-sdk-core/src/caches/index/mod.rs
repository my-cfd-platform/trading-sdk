pub trait TradingCacheIndexGenerator {
    fn get_id(&self) -> String;
    fn get_base(&self) -> Option<String>;
    fn get_quote(&self) -> Option<String>;
    fn get_collateral(&self) -> Option<String>;
    fn get_client_identification_index(&self) -> Option<String>;
    fn get_account_identification_index(&self) -> Option<String>;
}

mod engine_cache_index;
mod engine_cache_index_query;

pub use engine_cache_index::*;
pub use engine_cache_index_query::*;
