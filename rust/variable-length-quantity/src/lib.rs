use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|&number| encode(number)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .split_inclusive(|byte| byte & 0x80 == 0x00)
        .map(decode)
        .collect()
}

fn encode(mut number: u32) -> VecDeque<u8> {
    let mut result = VecDeque::new();

    loop {
        let byte = (number & 0x7F) as u8;
        number >>= 7;

        if result.is_empty() {
            result.push_back(byte);
        } else {
            result.push_front(byte | 0x80);
        }

        if number == 0 {
            break;
        }
    }

    result
}

fn decode(bytes: &[u8]) -> Result<u32, Error> {
    let mut result: u32 = 0;

    for (index, byte) in bytes.iter().rev().enumerate() {
        if index == 0 && (byte & 0x80 == 0x80) {
            return Err(Error::IncompleteNumber);
        }

        result = checked_shl((byte & 0x7F) as u32, 7 * index)
            .and_then(|byte_value| result.checked_add(byte_value))
            .ok_or(Error::Overflow)?;
    }
    Ok(result)
}

fn checked_shl(number: u32, bits: usize) -> Option<u32> {
    (bits == 0 || (bits < 32 && number < (1 << (32 - bits)))).then_some(number << bits)
}
