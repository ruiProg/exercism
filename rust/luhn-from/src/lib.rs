pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let digits: Option<Vec<_>> = self
            .0
            .chars()
            .rev()
            .filter(|&ch| ch != ' ')
            .map(|ch| ch.to_digit(10))
            .enumerate()
            .map(|(index, digit)| digit.map(|digit| digit_convert(index, digit)))
            .collect();

        digits
            .filter(|digits| digits.len() > 1)
            .map(|digits| digits.iter().sum::<u32>())
            .map_or(false, |digits_sum| digits_sum % 10 == 0)
    }
}

fn digit_convert(index: usize, digit: u32) -> u32 {
    match index % 2 {
        0 => digit,
        _ => {
            let digit = digit * 2;
            if digit > 9 {
                digit - 9
            } else {
                digit
            }
        }
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
