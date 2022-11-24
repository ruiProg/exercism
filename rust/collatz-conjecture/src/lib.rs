pub fn collatz(mut n: u64) -> Option<u64> {
    let mut iterations = 0;

    while n > 1 {
        n = match n % 2 {
            0 => n / 2,
            _ => n.checked_mul(3)?.checked_add(1)?,
        };
        iterations += 1;
    }

    (n == 1).then_some(iterations)
}
