use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    let mut words = HashMap::new();

    sentence
        .split(|ch: char| !ch.is_alphanumeric() && ch != '\'')
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches('\''))
        .map(|word| word.to_lowercase())
        .for_each(|word| {
            words
                .entry(word)
                .and_modify(|count| *count += 1)
                .or_insert(1_u32);
        });

    words
}
