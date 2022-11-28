pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &candidate)| row.iter().all(|&value| candidate >= value))
                .filter(|&(y, &candidate)| (0..input.len()).all(|x| candidate <= input[x][y]))
                .map(|(column_index, _)| (row_index, column_index))
                .collect::<Vec<_>>()
        })
        .collect()
}
