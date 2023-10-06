use trading_sdk_core::TradingCacheIndexGenerator;

use crate::MtPositionBaseData;

#[derive(Debug, Clone)]
pub struct MtPosition<T> {
    pub state: T,
    pub base_data: MtPositionBaseData,
}

impl<MtPositionActiveState> TradingCacheIndexGenerator for MtPosition<MtPositionActiveState> {
    fn get_id(&self) -> String {
        self.base_data.id.clone()
    }

    fn get_base(&self) -> Option<String> {
        Some(self.base_data.base.clone())
    }

    fn get_quote(&self) -> Option<String> {
        Some(self.base_data.quote.clone())
    }

    fn get_collateral(&self) -> Option<String> {
        Some(self.base_data.collateral.clone())
    }

    fn get_client_identification_index(&self) -> Option<String> {
        Some(self.base_data.trader_id.clone())
    }

    fn get_account_identification_index(&self) -> Option<String> {
        Some(self.base_data.account_id.clone())
    }
}
