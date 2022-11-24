pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            (0..row.len())
                .map(|column_index| char_at_cell(minefield, row_index, column_index))
                .collect()
        })
        .collect()
}

fn char_at_cell(minefield: &[&str], row_index: usize, column_index: usize) -> char {
    let cell_value = minefield[row_index].as_bytes()[column_index];

    (cell_value != b'*')
        .then(
            || match adjacent_mines_count(minefield, row_index, column_index) {
                0 => None,
                num_mines => char::from_digit(num_mines, 10),
            },
        )
        .flatten()
        .unwrap_or_else(|| cell_value.into())
}

fn adjacent_mines_count(minefield: &[&str], row_index: usize, column_index: usize) -> u32 {
    let min_row = row_index.saturating_sub(1);
    let min_column = column_index.saturating_sub(1);

    (min_row..=row_index + 1)
        .map(|current_row| {
            (min_column..=column_index + 1)
                .filter(|&current_column| {
                    current_row != row_index || current_column != column_index
                })
                .filter(|&current_column| has_mine_at(minefield, current_row, current_column))
                .count() as u32
        })
        .sum()
}

fn has_mine_at(minefield: &[&str], row_index: usize, column_index: usize) -> bool {
    minefield
        .get(row_index)
        .and_then(|row| row.as_bytes().get(column_index))
        .filter(|&&ch| ch == b'*')
        .is_some()
}
