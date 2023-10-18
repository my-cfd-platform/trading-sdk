use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MtPositionPendingStateType {
    BuyStop = 0,
    BuyLimit = 1,
    SellStop = 2,
    SellLimit = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtPositionPendingState {
    pub desire_price: f64,
    pub position_type: MtPositionPendingStateType,
}
