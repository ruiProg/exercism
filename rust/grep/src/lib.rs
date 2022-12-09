use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    show_line_numbers: bool,
    file_name_only: bool,
    case_insensitive: bool,
    inverted_match: bool,
    whole_line_match: bool,
}

impl Flags {
    pub fn new(input: &[&str]) -> Self {
        let mut flags = Self::default();

        input
            .iter()
            .filter(|flag_input| flag_input.starts_with('-'))
            .flat_map(|flag_input| &flag_input.as_bytes()[1..])
            .for_each(|option| match option {
                b'n' => flags.show_line_numbers = true,
                b'l' => flags.file_name_only = true,
                b'i' => flags.case_insensitive = true,
                b'v' => flags.inverted_match = true,
                b'x' => flags.whole_line_match = true,
                ch => panic!("Flag -{} not implemented", *ch as char),
            });

        flags
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let mut result = Vec::new();

    'file_loop: for file_path in files {
        let file_reader = BufReader::new(File::open(file_path)?);

        for (line_number, line) in file_reader.lines().enumerate() {
            let line = line?;

            if line_matches(&pattern, flags, &line) {
                if flags.file_name_only {
                    result.push(file_path.to_string());
                    continue 'file_loop;
                } else {
                    let mut line_result = line;

                    if flags.show_line_numbers {
                        line_result = format!("{}:{line_result}", line_number + 1);
                    }
                    if files.len() > 1 {
                        line_result = format!("{file_path}:{line_result}");
                    };

                    result.push(line_result);
                }
            }
        }
    }

    Ok(result)
}

fn line_matches(pattern: &str, flags: &Flags, line: &str) -> bool {
    let mut line = Cow::from(line);

    if flags.case_insensitive {
        *line.to_mut() = line.to_lowercase();
    }

    let line_matched = if flags.whole_line_match {
        line == pattern
    } else {
        line.contains(pattern)
    };

    if flags.inverted_match {
        !line_matched
    } else {
        line_matched
    }
}
