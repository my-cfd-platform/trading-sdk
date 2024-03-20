use crate::{get_position_total_invest, MtPosition, MtPositionActiveState, MtPositionBaseData};

pub fn calculate_position_topping_up(position: &MtPositionBaseData) -> Option<f64> {
    Some(position.topping_up_percent? * position.invest_amount / 100.0)
}

pub fn can_return_topping_up_funds(position: &MtPosition<MtPositionActiveState>) -> bool {
    let Some(margin_call) = position.base_data.margin_call_percent else {
        return false;
    };

    let Some(topping_up_sum) = position.state.topping_up else {
        return false;
    };

    if topping_up_sum < 0.01 {
        return false;
    }

    let Some(topping_up_amount) = calculate_position_topping_up(&position.base_data) else {
        return false;
    };

    let free_margin_without_last_topping_up =
        get_position_total_invest(position) + position.state.profit - topping_up_amount;
    let free_margin_without_last_topping_up_percent = free_margin_without_last_topping_up
        / (get_position_total_invest(position) - topping_up_amount)
        * 100.0;

    return 100.0 - free_margin_without_last_topping_up_percent <= margin_call;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestEntity;

    #[test]
    fn test_topping_up_calculation() {
        let mut position = MtPosition::generate_test_entity();

        position.base_data.invest_amount = 1000.0;
        position.base_data.topping_up_percent = Some(50.0);
        position.base_data.margin_call_percent = Some(40.0);

        let topping_up_amount = calculate_position_topping_up(&position.base_data).unwrap();

        assert_eq!(topping_up_amount, 500.0);
    }

    #[test]
    fn test_topping_up_calculation_topping_up_none() {
        let mut position = MtPosition::generate_test_entity();

        position.base_data.topping_up_percent = None;

        let topping_up_amount = calculate_position_topping_up(&position.base_data);

        assert_eq!(matches!(topping_up_amount, None), true);
    }

    #[test]
    fn test_can_return_funds() {
        let mut position = MtPosition::generate_test_entity();

        position.state.profit = 1000.0;
        position.base_data.topping_up_percent = Some(50.0);
        position.base_data.margin_call_percent = Some(40.0);
        position.state.topping_up = Some(100.0);

        let topping_up_amount = can_return_topping_up_funds(&position);

        assert_eq!(topping_up_amount, true);
    }
}
