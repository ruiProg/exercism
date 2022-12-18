use std::collections::HashMap;
use std::hash::Hash;
use std::thread;

use nohash_hasher::{IntMap, IsEnabled};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::default(),
        1..=LINES_THRESHOLD => frequency_lines(input)
            .iter()
            .map(|(letter, &value)| (letter.0, value))
            .collect(),
        _ => parallel_frequency(input, worker_count),
    }
}

fn parallel_frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::default();

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);

        for lines in input.chunks(ceil(input.len(), worker_count)) {
            handles.push(scope.spawn(|| frequency_lines(lines)));
        }

        for handle in handles {
            let letters = handle.join().unwrap();
            letters.iter().for_each(|(letter, &value)| {
                result
                    .entry(letter.0)
                    .and_modify(|total| *total += value)
                    .or_insert(value);
            });
        }
    });

    result
}

fn frequency_lines(input: &[&str]) -> IntMap<Letter, usize> {
    let mut map = IntMap::default();

    for line in input {
        for ch in line.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            map.entry(Letter(ch))
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }
    }

    map
}

fn ceil(lhs: usize, rhs: usize) -> usize {
    match lhs % rhs {
        0 => lhs / rhs,
        _ => lhs / rhs + 1,
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Letter(char);

impl IsEnabled for Letter {}

const LINES_THRESHOLD: usize = 1_000;
