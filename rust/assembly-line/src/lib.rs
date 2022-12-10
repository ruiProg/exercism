const HOUR_RATE: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        _ => 0.77,
    };

    speed as f64 * HOUR_RATE * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let minute_rate = production_rate_per_hour(speed) / 60.0;
    minute_rate as u32
}
