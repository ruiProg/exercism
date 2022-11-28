use std::iter;

#[derive(Debug)]
pub enum Error {
    InvalidInterval,
    InvalidTonic,
    TonicNotInScale,
}

pub struct Scale {
    scale: &'static [&'static str],
    start_pos: usize,
    intervals: Vec<usize>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = if Self::NEUTRAL_TONICS.contains(&tonic) || Self::SHARP_TONICS.contains(&tonic)
        {
            Ok(&Self::SHARP_SCALE)
        } else if Self::FLAT_TONICS.contains(&tonic) {
            Ok(&Self::FLAT_SCALE)
        } else {
            Err(Error::InvalidTonic)
        }?;

        let tonic = capitalize_first(tonic);
        let start_pos = scale
            .iter()
            .position(|&note| note == tonic)
            .ok_or(Error::TonicNotInScale)?;

        let intervals: Vec<_> = intervals
            .chars()
            .map(|ch| match ch {
                'm' => Ok(1),
                'M' => Ok(2),
                'A' => Ok(3),
                _ => Err(Error::InvalidInterval),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            scale,
            start_pos,
            intervals,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let intervals = "m".repeat(Self::SCALE_SIZE);
        Self::new(tonic, &intervals)
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut note_pos = self.start_pos;

        iter::once(&0)
            .chain(self.intervals.iter())
            .map(|increment| {
                note_pos = (note_pos + increment) % Self::SCALE_SIZE;
                self.scale[note_pos].to_string()
            })
            .collect()
    }

    const SCALE_SIZE: usize = 12;

    const SHARP_SCALE: [&str; Self::SCALE_SIZE] = [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];
    const FLAT_SCALE: [&str; Self::SCALE_SIZE] = [
        "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
    ];

    const NEUTRAL_TONICS: [&str; 2] = ["C", "a"];
    const SHARP_TONICS: [&str; Self::SCALE_SIZE] = [
        "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
    ];
    const FLAT_TONICS: [&str; Self::SCALE_SIZE] = [
        "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
    ];
}

fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
    }
}
