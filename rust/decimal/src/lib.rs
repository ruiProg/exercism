use std::{
    cmp::max,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(PartialEq, Eq, Debug)]
pub struct Decimal {
    number: BigInt,
    decimals: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        let mut number_parts = input.split('.');

        let whole_number = number_parts.next()?;
        let decimal_str = number_parts.next().unwrap_or("0").trim_end_matches('0');

        (number_parts.next().is_none())
            .then(|| {
                let number = BigInt::from_str(&format!("{whole_number}{decimal_str}")).ok()?;

                Some(Self {
                    number,
                    decimals: decimal_str.len(),
                })
            })
            .flatten()
    }

    fn to_number_decimals(&self, new_decimals: usize) -> Self {
        new_decimals
            .checked_sub(self.decimals)
            .map(|val| {
                let number_str = format!("{}{}", self.number, "0".repeat(val));
                Self {
                    number: BigInt::from_str(&number_str).unwrap(),
                    decimals: new_decimals,
                }
            })
            .expect("Decimals greater or equal to current")
    }

    fn normalize(&self) -> Self {
        let prev_str = self.number.to_string();
        let number_str: String = {
            let chars: Vec<_> = prev_str
                .chars()
                .rev()
                .enumerate()
                .skip_while(|&(index, ch)| ch == '0' && index < self.decimals)
                .map(|(_, ch)| ch)
                .collect();
            chars.iter().rev().collect()
        };

        Self {
            number: BigInt::from_str(&number_str).unwrap(),
            decimals: self.decimals - (prev_str.len() - number_str.len()),
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let common_decimals = max(self.decimals, rhs.decimals);
        Self {
            number: self.to_number_decimals(common_decimals).number
                + rhs.to_number_decimals(common_decimals).number,
            decimals: common_decimals,
        }
        .normalize()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let common_decimals = max(self.decimals, rhs.decimals);
        Self {
            number: self.to_number_decimals(common_decimals).number
                - rhs.to_number_decimals(common_decimals).number,
            decimals: common_decimals,
        }
        .normalize()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number * rhs.number,
            decimals: self.decimals + rhs.decimals,
        }
        .normalize()
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let common_decimals = max(self.decimals, other.decimals);

        self.to_number_decimals(common_decimals)
            .number
            .partial_cmp(&other.to_number_decimals(common_decimals).number)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let common_decimals = max(self.decimals, other.decimals);

        self.to_number_decimals(common_decimals)
            .number
            .cmp(&other.to_number_decimals(common_decimals).number)
    }
}
