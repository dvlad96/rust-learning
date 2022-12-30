// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR:i32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate:f64 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    return (f64::from(speed) * f64::from(CARS_PER_HOUR) * success_rate);
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod_per_hours:f64 = production_rate_per_hour(speed);

    return (prod_per_hours / 60.0) as u32;
}
