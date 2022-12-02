pub fn get_diamond(c: char) -> Vec<String> {
    let a_value: usize = u32::from('A') as usize;
    let dimensions: usize = (u32::from(c) as usize - a_value) * 2 + 1;
    let half_point = dimensions / 2;

    (0..dimensions)
        .map(|val| {
            let padding = half_point.abs_diff(val);
            let row_char = char::from((a_value + half_point - padding) as u8);
            let outer_space = " ".repeat(padding);

            if val == 0 || val == dimensions - 1 {
                format!("{outer_space}{row_char}{outer_space}")
            } else {
                let middle_space = " ".repeat(dimensions - (padding + 1) * 2);
                format!("{outer_space}{row_char}{middle_space}{row_char}{outer_space}")
            }
        })
        .collect()
}
