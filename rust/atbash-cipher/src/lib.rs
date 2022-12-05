/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter_map(transpose)
        .fold(Vec::<String>::new(), |mut acc, ch| {
            if let Some(chunk) = acc.last_mut().filter(|chunk| chunk.len() < CHUNK_SIZE) {
                chunk.push(ch);
            } else {
                let mut new_chunk = String::with_capacity(CHUNK_SIZE);
                new_chunk.push(ch);
                acc.push(new_chunk);
            }
            acc
        })
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(transpose).collect()
}

fn transpose(ch: char) -> Option<char> {
    if ch.is_ascii_lowercase() {
        let lower_bound = u32::from('a');
        let upper_bound = u32::from('z');
        let current_point = u32::from(ch);

        char::from_u32(upper_bound - (current_point - lower_bound))
    } else if ch.is_numeric() {
        Some(ch)
    } else {
        None
    }
}

const CHUNK_SIZE: usize = 5;
