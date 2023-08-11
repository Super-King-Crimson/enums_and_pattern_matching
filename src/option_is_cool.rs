#![allow(unused, dead_code)]

pub fn enumerate_coolness() {
    //Remember, Option can take two values: Some<T>, which holds a value of type T, and None, which holds no value.

    //It has a lot of methods, including:

    //is_some | is_some_and
    let valid_u8: Option<u8> = Some(10u8);
    let lost_u8: Option<u8> = None;

    assert_eq!(valid_u8.is_some(), true);
    assert_eq!(valid_u8.is_some_and(|x| x == 8), false);
    assert_eq!(lost_u8.is_some(), false);

    //Powerful tools like map/map_or/map_or_else to perform an operation on a value only if it exists
    let optional_u8: Option<u8> = None;
    let optional_u8_2 = Some(15u8);

    assert_eq!(optional_u8.map(|val| val.trailing_ones()), None);
    assert_eq!(optional_u8_2.map(|val| val.trailing_ones()), Some(4));
}
