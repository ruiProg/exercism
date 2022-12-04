/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Option<Vec<_>> = isbn
        .chars()
        .rev()
        .filter(|&ch| ch != '-')
        .enumerate()
        .map(|(index, ch)| {
            if ch == 'X' {
                (index == 0).then_some(10)
            } else {
                ch.to_digit(10).map(|val| val * (index as u32 + 1))
            }
        })
        .collect();

    digits
        .filter(|digits| digits.len() == 10)
        .map(|digits| digits.iter().sum::<u32>())
        .filter(|&sum| sum % 11 == 0)
        .is_some()
}
