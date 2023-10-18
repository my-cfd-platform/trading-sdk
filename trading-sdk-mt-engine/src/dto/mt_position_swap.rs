use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtPositionSwap {
    pub date: DateTimeAsMicroseconds,
    pub amount: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtPositionSwaps {
    pub swaps: Vec<MtPositionSwap>,
    pub total: f64,
}

impl Default for MtPositionSwaps {
    fn default() -> Self {
        Self {
            swaps: Vec::new(),
            total: 0.0,
        }
    }
}

impl MtPositionSwaps {
    pub fn add_swap(&mut self, amount: f64) {
        let swap = MtPositionSwap {
            date: DateTimeAsMicroseconds::now(),
            amount,
        };
        self.total += swap.amount;
        self.swaps.push(swap);
    }
}
