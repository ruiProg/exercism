use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use itertools::{Itertools, Permutations};

pub fn solve(input: &str) -> Option<Solution> {
    let mut problem = Alphametic::default();
    problem.parse(input).ok().and_then(|_| problem.solve())
}

type Solution = HashMap<char, u8>;

struct SolutionImpossible;

struct Constraint {
    addends: Vec<char>,
    result: char,
}

#[derive(Default)]
struct Alphametic {
    letters: BTreeMap<char, Range<u8>>,
    constraints: Vec<Constraint>,
}

impl Alphametic {
    fn parse(&mut self, input: &str) -> Result<(), SolutionImpossible> {
        let mut letter_iter = input.chars().rev().peekable();
        let mut index = 0;
        let mut is_addend = false;

        while let Some(letter) = letter_iter.next() {
            if letter.is_ascii_uppercase() {
                let leading = {
                    let next_letter = letter_iter.peek();
                    next_letter.is_none()
                        || next_letter.filter(|ch| ch.is_ascii_whitespace()).is_some()
                };
                self.add_letter(letter, leading);

                if is_addend {
                    self.add_addend(letter, index)?;
                } else {
                    self.add_sum_result(letter);
                }

                index += 1;
            } else {
                is_addend = true;
                index = 0;
            }
        }

        self.is_solvable()
    }

    fn solve(self) -> Option<Solution> {
        let mut candidate_iter = AlphameticCandidate::new(self.letters);

        'candidate: loop {
            let candidate = candidate_iter.next()?;
            let mut carry = 0;

            for constraint in &self.constraints {
                if let Some(addends_carry) = constraint_satisfied(constraint, &candidate, carry) {
                    carry = addends_carry;
                } else {
                    continue 'candidate;
                }
            }

            break Some(candidate);
        }
    }

    fn add_letter(&mut self, letter: char, leading: bool) {
        let start_index = leading.into();

        self.letters
            .entry(letter)
            .and_modify(|value| {
                if leading {
                    *value = 1..10;
                }
            })
            .or_insert(start_index..10);
    }

    fn add_addend(&mut self, letter: char, index: usize) -> Result<(), SolutionImpossible> {
        (index < self.constraints.len())
            .then(|| self.constraints[index].addends.push(letter))
            .ok_or(SolutionImpossible)
    }

    fn add_sum_result(&mut self, letter: char) {
        self.constraints.push(Constraint {
            addends: Vec::new(),
            result: letter,
        });
    }

    fn is_solvable(&self) -> Result<(), SolutionImpossible> {
        (self.letters.len() <= 10 && valid_constraints(&self.constraints))
            .then_some(())
            .ok_or(SolutionImpossible)
    }
}

fn valid_constraints(constraints: &[Constraint]) -> bool {
    constraints
        .first()
        .filter(|constraint| {
            constraint.addends.len() > 1 || Some(&constraint.result) == constraint.addends.first()
        })
        .is_some()
}

fn constraint_satisfied(constraint: &Constraint, candidate: &Solution, carry: u32) -> Option<u32> {
    let expected_result = *candidate.get(&constraint.result)? as u32;

    let addends_sum = carry
        + constraint
            .addends
            .iter()
            .map(|letter| candidate.get(letter).map(|&value| value as u32))
            .sum::<Option<u32>>()?;

    (expected_result == addends_sum % 10).then_some(addends_sum / 10)
}

struct AlphameticCandidate {
    permutations: Permutations<Range<u8>>,
    letters: BTreeMap<char, Range<u8>>,
}

impl AlphameticCandidate {
    fn new(letters: BTreeMap<char, Range<u8>>) -> Self {
        Self {
            permutations: (0..10).permutations(letters.len()),
            letters,
        }
    }
}

impl Iterator for AlphameticCandidate {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let values = self.letters.iter().zip(self.permutations.next()?);
            let possible_solution = values
                .clone()
                .all(|((_, letter_range), value)| letter_range.contains(&value));

            if possible_solution {
                let candidate: Solution = values
                    .map(|((&letter, _), value)| (letter, value))
                    .collect();
                break Some(candidate);
            }
        }
    }
}
