pub fn encode(n: u64) -> String {
    let mut number = n as usize;

    match number {
        0 => SPELLED_NUMBERS[number].to_string(),
        _ => {
            let mut chunks = Vec::new();
            let mut order = 0;

            while number > 0 {
                let group_number = encode_thousands(number).trim().to_string();

                if !group_number.is_empty() {
                    let group_number = format!("{group_number} {}", get_group_identifier(order));
                    chunks.push(group_number.trim().to_string());
                }

                number /= 1_000;
                order += 1;
            }

            chunks.reverse();
            chunks.join(" ")
        }
    }
}

fn encode_thousands(number: usize) -> String {
    match (number / 100) % 10 {
        0 => encode_tens(number % 100),
        val => format!(
            "{} hundred {}",
            SPELLED_NUMBERS[val],
            encode_tens(number % 100)
        ),
    }
}

fn encode_tens(number: usize) -> String {
    match number % 100 {
        1..=9 => SPELLED_NUMBERS[number].to_string(),
        val @ (10 | 11 | 12 | 13 | 15 | 18) => special_teen(val).to_string(),
        val @ 11..=19 => format!("{}teen", SPELLED_NUMBERS[val % 10]),
        _ => {
            let tens = match number / 10 {
                val @ (2 | 3 | 4 | 5 | 8) => special_ty(val).to_string(),
                val @ 1..=9 => format!("{}ty", SPELLED_NUMBERS[val]),
                _ => "".to_string(),
            };
            let remainder = number % 10;

            match remainder {
                0 => tens,
                _ => format!("{tens}-{}", SPELLED_NUMBERS[remainder]),
            }
        }
    }
}

fn get_group_identifier(order: usize) -> &'static str {
    match order {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => "",
    }
}

fn special_teen(number: usize) -> &'static str {
    match number {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        15 => "fifteen",
        18 => "eighteen",
        _ => "",
    }
}

fn special_ty(number: usize) -> &'static str {
    match number {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        8 => "eighty",
        _ => "",
    }
}

const SPELLED_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
