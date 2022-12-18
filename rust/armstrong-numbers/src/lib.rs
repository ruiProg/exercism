use std::iter::successors;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = successors(Some(num), |&rest| (rest >= 10).then_some(rest / 10))
        .map(|rest| rest % 10)
        .collect();
    let num_digits = digits.len() as u32;
    let transformed_num = digits
        .iter()
        .map(|rest| rest.pow(num_digits))
        .try_fold(0, u32::checked_add);

    Some(num) == transformed_num
}
