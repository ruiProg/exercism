pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|&num| is_multiple(num, factors)).sum()
}

fn is_multiple(num: u32, factors: &[u32]) -> bool {
    factors
        .iter()
        .filter(|&&factor| factor > 0)
        .any(|&factor| num % factor == 0)
}
