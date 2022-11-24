use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs {
        let operand = match input {
            CalculatorInput::Add => operation(&mut stack, i32::add)?,
            CalculatorInput::Subtract => operation(&mut stack, i32::sub)?,
            CalculatorInput::Multiply => operation(&mut stack, i32::mul)?,
            CalculatorInput::Divide => operation(&mut stack, i32::div)?,
            CalculatorInput::Value(number) => *number,
        };
        stack.push(operand);
    }

    (stack.len() == 1).then(|| stack[0])
}

fn operation(stack: &mut Vec<i32>, op: impl FnOnce(i32, i32) -> i32) -> Option<i32> {
    let rhs = stack.pop()?;
    let lhs = stack.pop()?;
    Some(op(lhs, rhs))
}
