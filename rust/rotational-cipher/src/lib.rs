pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|ch| transpose(ch, key)).collect()
}

fn transpose(ch: char, key: i8) -> char {
    if ch.is_ascii_alphabetic() {
        let lower_bound = if ch.is_ascii_lowercase() {
            u32::from('a')
        } else {
            u32::from('A')
        };
        let current_distance = u32::from(ch) - lower_bound;
        let rotation = (key + LIMIT as i8) as u32;

        char::from_u32(lower_bound + (current_distance + rotation) % LIMIT).unwrap_or(ch)
    } else {
        ch
    }
}

const LIMIT: u32 = 26;
