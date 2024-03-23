// Enum is a type that can have a fixed set of values
#[derive(Debug)]
enum CarColor {
    Red,
    Blue,
    Green,
    Black,
    White,
}

#[derive(Debug)]
enum GivenResult<T, E>{
    Success(T),
    Error(E),
}

#[derive(Debug)]
enum GivenOption<T>{
    Some(T),
    None,
}

fn create_car_color_white() -> CarColor {
    let my_car_color: CarColor = CarColor::White;
    my_car_color
}

fn check_num_below_five(num: u8) -> GivenResult<u8, String> {
    if num < 5 {
        GivenResult::Success(num)
    } else {
        GivenResult::Error("Number is not below 5".to_string())
    }
}

fn check_num_below_five_built_in(num: u8) -> Result<u8, String> {
    if num < 5 {
        Ok(num)
    } else {
        Err("Number is not below 5".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_enum(){
        let car_color: CarColor = create_car_color_white();
        dbg!(car_color);

        // let check_below_five: GivenResult<u8, String> = check_num_below_five(7);
        // dbg!(check_below_five);

        // let check_remainder: GivenOption<f32> = remainder_zero(12.0);
        // dbg!(check_remainder);

        let check_below_five: Result<u8, String> = check_num_below_five_built_in(7);
        dbg!(check_below_five);

        let check_remainder: Option<f32> = remainder_zero_built_in(12.0);
        dbg!(check_remainder);
    }
}

