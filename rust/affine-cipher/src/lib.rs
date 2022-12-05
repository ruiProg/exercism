/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    (gcd(a, ALPHA_M) == 1)
        .then(|| {
            plaintext
                .to_ascii_lowercase()
                .chars()
                .filter_map(|ch| transpose(ch, a, b, encrypt))
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
        })
        .ok_or(AffineCipherError::NotCoprime(a))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    (gcd(a, ALPHA_M) == 1)
        .then(|| {
            ciphertext
                .chars()
                .filter_map(|ch| transpose(ch, a, b, decrypt))
                .collect()
        })
        .ok_or(AffineCipherError::NotCoprime(a))
}

const ALPHA_M: i32 = 26;
const CHUNK_SIZE: usize = 5;

fn gcd(a: i32, b: i32) -> i32 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) if a > b => gcd(a, a % b),
        (a, b) => gcd(b, b % a),
    }
}

fn mmi(a: i32, b: i32) -> i32 {
    let val = a % b;
    (1..).find(|candidate| val * candidate % b == 1).unwrap()
}

fn encrypt(a: i32, b: i32, x: i32) -> i32 {
    (a * x + b).rem_euclid(ALPHA_M)
}

fn decrypt(a: i32, b: i32, x: i32) -> i32 {
    (mmi(a, ALPHA_M) * (x - b)).rem_euclid(ALPHA_M)
}

fn transpose(
    ch: char,
    a: i32,
    b: i32,
    transform_fn: impl Fn(i32, i32, i32) -> i32,
) -> Option<char> {
    if ch.is_ascii_lowercase() {
        let lower_bound = u32::from('a');
        let x = (u32::from(ch) - lower_bound) as i32;
        let value = transform_fn(a, b, x);

        char::from_u32(lower_bound + value as u32)
    } else if ch.is_numeric() {
        Some(ch)
    } else {
        None
    }
}
