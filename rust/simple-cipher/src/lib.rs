use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    translate(key, s, ShiftOperation::Encode)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    translate(key, s, ShiftOperation::Decode)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = (0..RAND_CHARACTERS)
        .map(|_| rng.gen_range('a'..='z'))
        .collect();

    let encoded_message = encode(&key, s).unwrap();
    (key, encoded_message)
}

fn translate(key: &str, s: &str, operation: ShiftOperation) -> Option<String> {
    (!key.is_empty())
        .then(|| {
            s.chars()
                .zip(key.chars().cycle())
                .map(|(ch, key)| transpose(ch, key, operation))
                .collect()
        })
        .flatten()
}

fn transpose(ch: char, key: char, operation: ShiftOperation) -> Option<char> {
    (ch.is_ascii_lowercase() && key.is_ascii_lowercase())
        .then(|| {
            let lower_bound = u32::from('a');
            let current_distance = u32::from(ch) - lower_bound;

            let rotation = u32::from(key) - lower_bound;
            let rotation = match operation {
                ShiftOperation::Encode => rotation,
                ShiftOperation::Decode => LIMIT - rotation,
            };

            char::from_u32(lower_bound + (current_distance + rotation) % LIMIT)
        })
        .flatten()
}

const LIMIT: u32 = 26;
const RAND_CHARACTERS: usize = 1024;

#[derive(Clone, Copy)]
enum ShiftOperation {
    Encode,
    Decode,
}
