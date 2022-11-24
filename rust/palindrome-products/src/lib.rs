use std::iter::successors;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        is_palindrome(value).then_some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    smallest_palindrome(min, max).zip(biggest_palindrome(min, max))
}

fn smallest_palindrome(min_factor: u64, max_factor: u64) -> Option<Palindrome> {
    find_palindrome(
        min_factor,
        min_factor..=max_factor,
        |first_factor| min_factor..=first_factor,
        u64::gt,
    )
}

fn biggest_palindrome(min_factor: u64, max_factor: u64) -> Option<Palindrome> {
    find_palindrome(
        max_factor,
        (min_factor..=max_factor).rev(),
        |first_factor| (first_factor..=max_factor).rev(),
        u64::lt,
    )
}

fn find_palindrome<IterFunction>(
    start_factor: u64,
    first_factor_iter: impl Iterator<Item = u64>,
    second_factor_iter_fn: IterFunction,
    stop_fn: impl Fn(&u64, &u64) -> bool,
) -> Option<Palindrome>
where
    IterFunction: WrappedFunction<u64>,
    <IterFunction as WrappedFunction<u64>>::ReturnType: Iterator<Item = u64>,
{
    let factors_exhausted =
        |current: Option<u64>, candidate| current.filter(|&x| stop_fn(&candidate, &x)).is_some();
    let mut current = None;

    for first_factor in first_factor_iter {
        if factors_exhausted(current, first_factor * start_factor) {
            break;
        }

        for second_factor in second_factor_iter_fn.call(first_factor) {
            let candidate = first_factor * second_factor;

            if factors_exhausted(current, candidate) {
                break;
            }

            if is_palindrome(candidate) {
                current = Some(candidate);
            }
        }
    }

    current.map(Palindrome)
}

fn is_palindrome(value: u64) -> bool {
    let digits: Vec<_> = successors(Some(value), |&rest| (rest >= 10).then_some(rest / 10))
        .map(|rest| rest % 10)
        .collect();

    Iterator::zip(digits.iter(), digits.iter().rev())
        .take(digits.len() / 2)
        .all(|(&front, &back)| front == back)
}

trait WrappedFunction<T> {
    type ReturnType;

    fn call(&self, value: T) -> Self::ReturnType;
}

impl<T, F, R> WrappedFunction<T> for F
where
    F: Fn(T) -> R,
{
    type ReturnType = R;

    fn call(&self, value: T) -> Self::ReturnType {
        self(value)
    }
}
