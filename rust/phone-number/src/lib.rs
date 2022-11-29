pub fn number(user_number: &str) -> Option<String> {
    let numbers: Vec<_> = user_number.chars().filter(|ch| ch.is_numeric()).collect();
    let first_digit = numbers.first()?;

    let window = if *first_digit == '1' && numbers.len() == 11 {
        &numbers[1..]
    } else {
        &numbers[0..]
    };

    (window.len() == 10 && window[0] > '1' && window[3] > '1').then(|| window.iter().collect())
}
