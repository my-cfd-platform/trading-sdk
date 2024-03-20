use crate::{MtPosition, MtPositionActiveState};

pub fn apply_position_topping_up(
    topping_up_amount: f64,
    position: &mut MtPosition<MtPositionActiveState>,
) {
    if let Some(topping_up) = &position.state.topping_up {
        position.state.topping_up = Some(topping_up + topping_up_amount);
    } else {
        position.state.topping_up = Some(topping_up_amount);
    }
}

pub fn return_topping_up(topping_up_amount: f64, position: &mut MtPosition<MtPositionActiveState>) {
    if let Some(topping_up) = &position.state.topping_up {
        if topping_up < &topping_up_amount {
            panic!(
                "Topping up amount is less than requested, cant return for position: {}",
                position.base_data.id
            )
        }

        position.state.topping_up = Some(topping_up - topping_up_amount);
    } else {
        panic!(
            "Topping up amount is not set, cant return for position: {}",
            position.base_data.id
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestEntity;

    #[test]
    fn test_apply_position_topping_up() {
        let mut position = MtPosition::generate_test_entity();

        position.state.topping_up = Some(10.0);
        apply_position_topping_up(10.0, &mut position);

        assert_eq!(position.state.topping_up, Some(20.0));
    }

    #[test]
    fn test_apply_position_topping_up_none() {
        let mut position = MtPosition::generate_test_entity();

        position.state.topping_up = None;
        apply_position_topping_up(10.0, &mut position);

        assert_eq!(position.state.topping_up, Some(10.0));
    }

    #[test]
    fn test_return_topping_up() {
        let mut position = MtPosition::generate_test_entity();
        position.state.topping_up = Some(10.0);
        return_topping_up(5.0, &mut position);

        assert_eq!(position.state.topping_up, Some(5.0));
    }

    #[test]
    #[should_panic]
    fn test_return_topping_up_panic_no_funds() {
        let mut position = MtPosition::generate_test_entity();
        position.state.topping_up = Some(3.0);
        return_topping_up(5.0, &mut position);
    }

    #[test]
    #[should_panic]
    fn test_return_topping_up_panic_no_topping_up() {
        let mut position = MtPosition::generate_test_entity();
        position.state.topping_up = None;
        return_topping_up(5.0, &mut position);
    }
}
