#[derive(Debug, Clone)]
pub enum MtPositionPendingStateType {
    BuyStop = 0,
    BuyLimit = 1,
    SellStop = 2,
    SellLimit = 3,
}

#[derive(Debug, Clone)]
pub struct MtPositionPendingState {
    pub desire_price: f64,
    pub position_type: MtPositionPendingStateType,
}
