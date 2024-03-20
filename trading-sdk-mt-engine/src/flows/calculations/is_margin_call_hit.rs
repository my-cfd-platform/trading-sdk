use crate::{MtPosition, MtPositionActiveState};

pub fn update_margin_call_hit(position: &mut MtPosition<MtPositionActiveState>) -> bool {
    let is_hit = is_margin_call_hit(position);
    let is_change = is_hit != position.state.is_margin_call_hit;
    position.state.is_margin_call_hit = is_hit;

    is_change
}

pub fn is_margin_call_hit(position: &MtPosition<MtPositionActiveState>) -> bool {
    let Some(margin_call_percent) = position.base_data.margin_call_percent else {
        return false;
    };

    let free_margin = get_position_total_invest(position) + position.state.profit;
    let free_margin_percent = free_margin / get_position_total_invest(position) * 100.0;
    100.0 - free_margin_percent >= margin_call_percent
}

pub fn get_position_total_invest(position: &MtPosition<MtPositionActiveState>) -> f64 {
    let Some(topping_up) = &position.state.topping_up else {
        return position.base_data.invest_amount;
    };

    position.base_data.invest_amount + topping_up
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestEntity;

    #[test]
    fn test_margin_call_none() {
        let mut position = MtPosition::generate_test_entity();
        position.base_data.margin_call_percent = None;

        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, false);
    }

    #[test]
    fn test_margin_call_hit() {
        let mut position = MtPosition::generate_test_entity();
        
        position.state.profit = -100.0;
        position.base_data.margin_call_percent = Some(10.0);

        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, true);
    }

    #[test]
    fn test_margin_call_not_hit() {
        let mut position = MtPosition::generate_test_entity();
        
        position.base_data.invest_amount = 1000.0;
        position.state.profit = -100.0;
        position.base_data.margin_call_percent = Some(90.0);

        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, false);
    }

    #[test]
    fn test_margin_call_not_hit_with_topping_up() {
        let mut position = MtPosition::generate_test_entity();
        
        position.base_data.invest_amount = 1.0;
        position.state.topping_up = Some(1000.0);
        position.state.profit = -100.0;
        position.base_data.margin_call_percent = Some(90.0);

        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, false);
    }

    #[test]
    fn test_margin_call_direct_hit() {
        let mut position = MtPosition::generate_test_entity();
        
        position.base_data.invest_amount = 100.0;
        position.state.profit = -90.0;
        position.base_data.margin_call_percent = Some(90.0);

        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, true);


        position.base_data.invest_amount = 100.0;
        position.state.profit = -89.0;
        position.base_data.margin_call_percent = Some(90.0);

        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, false);
    }

    #[test]
    fn test_margin_call_direct_hit_topping_up() {
        let mut position = MtPosition::generate_test_entity();
        
        position.base_data.invest_amount = 1.0;
        position.state.topping_up = Some(99.0);
        position.state.profit = -90.0;
        position.base_data.margin_call_percent = Some(90.0);
        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, true);

        position.base_data.invest_amount = 1.0;
        position.state.topping_up = Some(99.0);
        position.state.profit = -89.0;
        position.base_data.margin_call_percent = Some(90.0);
        update_margin_call_hit(&mut position);
        assert_eq!(position.state.is_margin_call_hit, false);
    }
}
