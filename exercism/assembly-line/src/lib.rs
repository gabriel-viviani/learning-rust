// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_SPEED: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        0 ..= 4 => 1.0,
        5 ..= 8 => 0.9,
        9 | 10  => 0.77,
        11.. => panic!("Invalid speed range")
    };

    let speed = speed as u32;
    let speed = (BASE_SPEED * speed);
    let speed = speed as f64;

    return speed * success_rate;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod_rate: f64 = production_rate_per_hour(speed);

    return prod_rate as u32 / 60;
}
