pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();

    (2..)
        .filter(|&val| {
            let is_prime = primes.iter().all(|&prime| val % prime != 0);
            if is_prime {
                primes.push(val);
            }
            is_prime
        })
        .nth(n as usize)
        .unwrap()
}
