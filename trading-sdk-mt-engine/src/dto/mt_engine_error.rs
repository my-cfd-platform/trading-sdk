use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MtEngineError {
    NoLiquidity,
    PositionNotFound,
}
