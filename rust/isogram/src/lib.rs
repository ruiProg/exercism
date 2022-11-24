use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();

    candidate
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .all(|ch| letters.insert(ch))
}
