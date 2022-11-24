pub fn verse(n: u32) -> String {
    let current_amount = number_of_bottles(n);
    let upper_current_amount = capitalize_first(&current_amount);
    let action = next_action(n);
    let next_amount = number_of_bottles(n.checked_sub(1).unwrap_or(99));

    format!("{upper_current_amount} of beer on the wall, {current_amount} of beer.\n{action}, {next_amount} of beer on the wall.\n")
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<_> = (end..=start).rev().map(verse).collect();
    verses.join("\n")
}

fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
    }
}

fn number_of_bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".into(),
        1 => "1 bottle".into(),
        _ => format!("{n} bottles"),
    }
}

fn next_action(n: u32) -> &'static str {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
}
