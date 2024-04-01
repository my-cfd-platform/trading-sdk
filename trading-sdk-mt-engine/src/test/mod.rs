pub trait TestEntity {
    fn generate_test_entity() -> Self;
}

#[cfg(test)]
mod test {
    use crate::{apply_position_topping_up, calculate_position_topping_up, can_return_topping_up_funds, is_margin_call_hit, MtPosition, TestEntity};

    #[test]
    fn test_marin_call_after_topping_up() {
        let mut position = MtPosition::generate_test_entity();

        position.base_data.invest_amount = 1000.0;
        position.state.profit = -500.0;
        position.base_data.margin_call_percent = Some(50.0);
        position.base_data.topping_up_percent = Some(50.0);

        let mkh = is_margin_call_hit(&position);
        let mka = calculate_position_topping_up(&position.base_data);
    
        apply_position_topping_up(500.0, &mut position);

        println!("1) {} - {:?}", mkh, mka);


        let mkh = is_margin_call_hit(&position);
        let mka = calculate_position_topping_up(&position.base_data);
        let crt = can_return_topping_up_funds(&position);
        println!("2) {} - {:?} - {}", mkh, mka, crt);

        position.state.profit = -499.0;
        
        let mkh = is_margin_call_hit(&position);
        let mka = calculate_position_topping_up(&position.base_data);
        let crt = can_return_topping_up_funds(&position);
        println!("3) {} - {:?} - {}", mkh, mka, crt);
    }
}
