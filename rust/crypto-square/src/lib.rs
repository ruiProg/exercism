use std::iter::once;

pub fn encrypt(input: &str) -> String {
    let normalized_input: Vec<_> = input
        .chars()
        .filter_map(|ch| ch.is_ascii_alphanumeric().then(|| ch.to_ascii_lowercase()))
        .collect();

    let rows = f64::sqrt(normalized_input.len() as f64).floor() as usize;
    let columns = if rows.pow(2) == normalized_input.len() {
        rows
    } else {
        rows + 1
    };

    (0..columns)
        .map(|index| {
            normalized_input
                .iter()
                .skip(index)
                .step_by(columns)
                .chain(once(&' '))
                .take(rows)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
