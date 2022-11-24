pub fn square(s: u32) -> u64 {
    (1..=64)
        .contains(&s)
        .then(|| 2_u64.pow(s - 1))
        .expect("Square must be between 1 and 64")
}

pub fn total() -> u64 {
    u64::MAX
}
