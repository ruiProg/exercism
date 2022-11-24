use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let anagram_letters = count_letters(&lowercase_word);

    possible_anagrams
        .iter()
        .map(|candidate| (candidate, candidate.to_lowercase()))
        .filter(|(_, lowercase_candidate)| *lowercase_candidate != lowercase_word)
        .filter(|(_, lowercase_candidate)| is_anagram(lowercase_candidate, &anagram_letters))
        .map(|(&candidate, _)| candidate)
        .collect()
}

fn count_letters(word: &str) -> HashMap<char, u8> {
    let mut entries = HashMap::new();

    for ch in word.chars() {
        let count = entries.entry(ch).or_insert(0);
        *count += 1;
    }

    entries
}

fn is_anagram(candidate: &str, anagram_letters: &HashMap<char, u8>) -> bool {
    count_letters(candidate) == *anagram_letters
}
