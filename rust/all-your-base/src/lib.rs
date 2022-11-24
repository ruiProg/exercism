use std::ops::ControlFlow;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    parse_number(number, from_base).map(|number| base_digits(number, to_base))
}

fn parse_number(number: &[u32], base: u32) -> Result<u32, Error> {
    let number = number
        .iter()
        .rev()
        .try_fold((0, 0), |(index, acc), &digit| {
            if digit < base {
                ControlFlow::Continue((index + 1, acc + digit * base.pow(index)))
            } else {
                ControlFlow::Break(digit)
            }
        });

    match number {
        ControlFlow::Break(digit) => Err(Error::InvalidDigit(digit)),
        ControlFlow::Continue((_, acc)) => Ok(acc),
    }
}

fn base_digits(mut number: u32, base: u32) -> Vec<u32> {
    let mut digits = Vec::new();

    while number > 0 {
        digits.push(number % base);
        number /= base;
    }

    digits.reverse();
    digits.is_empty().then(|| vec![0]).unwrap_or(digits)
}
