pub fn encode(source: &str) -> String {
    let mut source = source.chars().peekable();
    let mut encoded_str = String::new();
    let mut element_count = 0;

    while let Some(element) = source.next() {
        element_count += 1;

        if source.peek() != Some(&element) {
            if element_count > 1 {
                encoded_str.push_str(&element_count.to_string());
            }
            encoded_str.push(element);
            element_count = 0;
        }
    }

    encoded_str
}

pub fn decode(source: &str) -> String {
    let mut decoded_str = String::new();
    let mut element_count = String::new();

    for element in source.chars() {
        if element.is_numeric() {
            element_count.push(element);
        } else {
            let count = element_count.parse().unwrap_or(1);
            decoded_str.push_str(&element.to_string().repeat(count));
            element_count.clear();
        }
    }

    decoded_str
}
