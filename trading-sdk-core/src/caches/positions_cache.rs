use std::collections::HashMap;

use crate::{EngineCacheQueryBuilder, TradingCacheIndex, TradingCacheIndexGenerator};
pub struct PositionsCache<T: TradingCacheIndexGenerator> {
    pub indexes: TradingCacheIndex,
    pub positions: HashMap<String, T>,
}

impl<T: TradingCacheIndexGenerator> PositionsCache<T> {
    pub fn new() -> Self {
        Self {
            indexes: TradingCacheIndex::new(),
            positions: HashMap::new(),
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<&T> {
        self.positions.get(id)
    }

    pub fn add_position(&mut self, position: T) {
        self.indexes.add_index(&position);
        self.positions.insert(position.get_id(), position);
    }

    pub fn remove_position(&mut self, id: &str) -> Option<T> {
        self.indexes.remove_index(id);
        self.positions.remove(id)
    }

    pub fn query_positions(&self, query: EngineCacheQueryBuilder) -> Vec<&T> {
        let indexes = self.indexes.query(&query);
        let mut result = vec![];

        for index in indexes {
            if let Some(position) = self.positions.get(index.as_ref()) {
                result.push(position);
            }
        }

        return result;
    }

    pub fn query_and_select_remove(
        &mut self,
        query: EngineCacheQueryBuilder,
        is_remove: impl Fn(&T) -> bool,
    ) -> Vec<T> {
        let indexes = self.indexes.query(&query);

        let mut to_return = vec![];

        for index in indexes {
            if let Some(position) = self.positions.get(index.as_ref()) {
                if is_remove(position) {
                    to_return.push(self.remove_position(&position.get_id()).unwrap());
                }
            }
        }

        return to_return;
    }

    pub fn update_position(
        &mut self,
        id: &str,
        update_command: impl Fn(Option<&mut T>) -> Option<T>,
    ) -> Option<T> {
        let position = self.positions.get_mut(id);
        update_command(position)
    }

    pub fn update_positions<F>(
        &mut self,
        query: EngineCacheQueryBuilder,
        update_command: impl Fn(&mut T) -> Option<F>,
    ) -> Vec<F> {
        let indexes = self.indexes.query(&query);
        let mut result = vec![];
        for index in indexes {
            if let Some(position) = self.positions.get_mut(index.as_ref()) {
                let update_result = update_command(position);
                if let Some(update_result) = update_result {
                    result.push(update_result);
                };
            }
        }

        return result;
    }
}
