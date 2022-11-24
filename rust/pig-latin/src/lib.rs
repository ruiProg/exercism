pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if word.starts_with(|ch| vowels.contains(&ch))
        || word.starts_with("xr")
        || word.starts_with("yt")
    {
        format!("{word}ay")
    } else if let Some(mut cluster_index) = word.find(|ch| vowels.contains(&ch) || ch == 'y') {
        if &word[..=0] == "y" || &word[cluster_index - 1..=cluster_index] == "qu" {
            cluster_index += 1;
        }
        format!("{}{}ay", &word[cluster_index..], &word[0..cluster_index])
    } else {
        "".into()
    }
}
