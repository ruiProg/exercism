pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|ch: char| ch.is_whitespace() || ch == '-')
        .filter_map(word_capital_letters)
        .collect::<String>()
        .to_uppercase()
}

fn word_capital_letters(word: &str) -> Option<String> {
    let first_letter_index = word.find(char::is_alphabetic)?;
    let first_letter = word.chars().nth(first_letter_index)?;

    let all_uppercase = word
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .all(char::is_uppercase);

    if !all_uppercase {
        let rest_letters = word[first_letter_index + 1..]
            .chars()
            .filter(|ch| ch.is_alphabetic() && ch.is_uppercase())
            .collect::<String>();

        Some(format!("{first_letter}{rest_letters}"))
    } else {
        Some(first_letter.into())
    }
}
