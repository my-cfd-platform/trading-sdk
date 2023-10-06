#[derive(Debug, Clone)]
pub enum MtPositionCloseReason{
    ClientCommand = 0,
    StopOut = 1,
    TakeProfit = 2,
    StopLoss = 3,
    ForceClose = 4,
}