use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    proteins: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.proteins.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        (0..ceil(rna.len(), Self::CODON_LENGTH))
            .map(|index| rna.get(index * Self::CODON_LENGTH..(index + 1) * Self::CODON_LENGTH))
            .map(|codon| codon.and_then(|codon| self.name_for(codon)))
            .take_while(|codon| !matches!(codon, Some(Self::STOP_CODON)))
            .collect()
    }

    const CODON_LENGTH: usize = 3;
    const STOP_CODON: &'static str = "stop codon";
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        proteins: pairs.into_iter().collect(),
    }
}

fn ceil(lhs: usize, rhs: usize) -> usize {
    if lhs % rhs == 0 {
        lhs / rhs
    } else {
        lhs / rhs + 1
    }
}
