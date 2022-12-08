#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        Ok(1)
    } else {
        let digits = string_digits
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .map(|val| val as u64)
                    .ok_or(Error::InvalidDigit(ch))
            })
            .collect::<Result<Vec<_>, _>>()?;

        digits
            .windows(span)
            .map(|slice| slice.iter().product())
            .max()
            .ok_or(Error::SpanTooLong)
    }
}
