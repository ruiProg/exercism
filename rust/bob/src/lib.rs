pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_yelling = {
        let alpha_letters: Vec<_> = message.chars().filter(|ch| ch.is_alphabetic()).collect();

        !alpha_letters.is_empty() && alpha_letters.iter().all(|ch| ch.is_uppercase())
    };

    match (message.chars().last(), is_yelling) {
        (Some('?'), true) => "Calm down, I know what I'm doing!",
        (Some('?'), _) => "Sure.",
        (Some(_), true) => "Whoa, chill out!",
        (Some(_), _) => "Whatever.",
        (None, _) => "Fine. Be that way!",
    }
}
