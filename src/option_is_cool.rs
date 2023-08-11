#![allow(unused, dead_code)]

pub fn enumerate_coolness() {
    //Remember, Option can take two values: Some<T>, which holds a value of type T, and None, which holds no value.

    //It has a lot of methods, including:

    //is_some | is_some_and
    let valid_u8: Option<u8> = Some(10u8);
    let lost_u8: Option<u8> = None;
}

fn add_some_u8(sm: Option<u8>) -> Option<u8> {
    //COME BACK WHEN YOU LEARN MATCH STATEMENTS!
    Some(4)
}
