use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let numerals: String = [1_000, 100, 10, 1]
            .iter()
            .enumerate()
            .filter_map(|(index, &val)| Self::encode((self.0 / val) % 10, index))
            .collect();

        f.write_str(&numerals)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        (1..4000)
            .contains(&num)
            .then_some(Self(num))
            .expect("Number not in range (0, 4000)")
    }
}

impl Roman {
    const NUMERALS: [[char; 2]; 4] = [['M', ' '], ['C', 'D'], ['X', 'L'], ['I', 'V']];
    const ONE_INDEX: usize = 0;
    const FIVE_INDEX: usize = 1;

    fn encode(val: u32, order: usize) -> Option<String> {
        match val {
            count @ (1 | 2 | 3) => Some(
                Self::NUMERALS[order][Self::ONE_INDEX]
                    .to_string()
                    .repeat(count as usize),
            ),
            4 => Some(format!(
                "{}{}",
                Self::NUMERALS[order][Self::ONE_INDEX],
                Self::NUMERALS[order][Self::FIVE_INDEX],
            )),
            count @ (5 | 6 | 7 | 8) => Some(format!(
                "{}{}",
                Self::NUMERALS[order][Self::FIVE_INDEX],
                Self::NUMERALS[order][Self::ONE_INDEX]
                    .to_string()
                    .repeat(count as usize - 5),
            )),
            9 => Some(format!(
                "{}{}",
                Self::NUMERALS[order][Self::ONE_INDEX],
                Self::NUMERALS[order - 1][Self::ONE_INDEX],
            )),
            _ => None,
        }
    }
}
