pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.0).fold(Vec::new(), |mut triangles, row| {
            let new_row = triangles
                .last()
                .map(|last_row| triangle_row(last_row, row))
                .unwrap_or_else(|| vec![1]);
            triangles.push(new_row);
            triangles
        })
    }
}

fn triangle_row(last_row: &[u32], row: u32) -> Vec<u32> {
    (0..row)
        .map(|column| {
            (1..row - 1)
                .contains(&column)
                .then(|| last_row[column as usize - 1] + last_row[column as usize])
                .unwrap_or(1)
        })
        .collect()
}
