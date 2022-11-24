pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut current_factor: u64 = 2;

    while n >= current_factor.pow(2) {
        while n % current_factor == 0 {
            factors.push(current_factor);
            n /= current_factor;
        }
        current_factor += 1;
    }

    if n > 1 {
        factors.push(n);
    }
    factors
}
