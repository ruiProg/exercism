pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits.len().checked_sub(len).map_or(Vec::new(), |windows| {
        (0..=windows)
            .map(|index| digits[index..len + index].into())
            .collect()
    })
}
