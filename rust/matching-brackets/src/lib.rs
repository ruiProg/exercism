use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let bracket_pairs = {
        let mut pairs = HashMap::new();
        pairs.insert('(', ')');
        pairs.insert('[', ']');
        pairs.insert('{', '}');
        pairs
    };
    let mut stack = Vec::new();

    for ch in string.chars() {
        if bracket_pairs.contains_key(&ch) {
            stack.push(ch);
        }
        if let Some(&current_bracket) = closing_bracket(&bracket_pairs, ch) {
            let Some(opening_bracket) = stack.pop() else {
                return false;
            };
            if bracket_pairs.get(&opening_bracket) != Some(&current_bracket) {
                return false;
            };
        }
    }
    stack.is_empty()
}

fn closing_bracket(bracket_pairs: &HashMap<char, char>, ch: char) -> Option<&char> {
    bracket_pairs.values().find(|&&bracket| bracket == ch)
}
