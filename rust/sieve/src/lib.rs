pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper_bound = upper_bound as usize;
    let mut candidates = vec![true; upper_bound + 1];
    let upper_iter = f64::sqrt(upper_bound as f64).ceil() as usize;

    for val in 2..=upper_iter {
        if candidates[val] {
            (2..)
                .map_while(|index| {
                    let next_val = index * val;
                    (next_val <= upper_bound).then_some(next_val)
                })
                .for_each(|val| candidates[val] = false);
        }
    }

    candidates
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(val, &is_prime)| is_prime.then_some(val as u64))
        .collect()
}
