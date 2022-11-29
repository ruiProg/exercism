pub fn answer(command: &str) -> Option<i32> {
    let (question, rest) = split_at(command, is_numeric_or_minus)?;
    let (result, tail) = expression(rest)?;
    (question == "What is" && tail == "?").then_some(result)
}

fn expression(command: &str) -> Option<(i32, &str)> {
    let (mut value, mut command) = identity(command)?;

    while let Some((operation, inner_command)) = find_operation(command) {
        let (other_value, mut inner_command) = identity(inner_command)?;
        value = match operation {
            Operation::Addition => value.checked_add(other_value)?,
            Operation::Subtraction => value.checked_sub(other_value)?,
            Operation::Multiplication => value.checked_mul(other_value)?,
            Operation::Division => value.checked_div(other_value)?,
            Operation::Exponential => {
                inner_command = valid_exponent(other_value, inner_command)?;
                value.checked_pow(other_value as u32)?
            }
        };
        command = inner_command;
    }

    Some((value, command))
}

fn identity(command: &str) -> Option<(i32, &str)> {
    let (number, rest) = split_at(command, |ch: char| !is_numeric_or_minus(ch))?;
    (number.parse::<i32>()).map(|value| (value, rest)).ok()
}

fn find_operation(command: &str) -> Option<(Operation, &str)> {
    let (operator, rest) = split_at(command, is_numeric_or_minus)?;
    let operation = match operator {
        "plus" => Some(Operation::Addition),
        "minus" => Some(Operation::Subtraction),
        "multiplied by" => Some(Operation::Multiplication),
        "divided by" => Some(Operation::Division),
        "raised to the" => Some(Operation::Exponential),
        _ => None,
    };
    operation.map(|operation| (operation, rest))
}

fn is_numeric_or_minus(ch: char) -> bool {
    ch.is_numeric() || ch == '-'
}

fn valid_exponent(value: i32, command: &str) -> Option<&str> {
    let value: u32 = value.try_into().ok()?;
    let expected_suffix = number_suffix(value);
    let (suffix, rest) = split_at(command, char::is_whitespace)?;

    let command = (expected_suffix == suffix).then_some(rest)?;
    let (power_str, rest) = split_at(command, |ch: char| !ch.is_alphabetic())?;

    (power_str == "power").then_some(rest)
}

fn number_suffix(value: u32) -> &'static str {
    match (value % 10, value % 100) {
        (1, remainder) if remainder != 11 => "st",
        (2, remainder) if remainder != 12 => "nd",
        (3, remainder) if remainder != 13 => "rd",
        _ => "th",
    }
}

fn split_at(command: &str, separator_fn: impl Fn(char) -> bool) -> Option<(&str, &str)> {
    command
        .find(separator_fn)
        .map(|split_index| (command[..split_index].trim(), command[split_index..].trim()))
}

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponential,
}
