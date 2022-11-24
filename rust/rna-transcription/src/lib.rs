#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<Nucleotide>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<Nucleotide>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        acid_from_string(dna, &Self::NUCLEOTIDES).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        let nucleotides: Vec<_> = self
            .0
            .iter()
            .map(|nucleotide| match nucleotide {
                Nucleotide::Adenine => Nucleotide::Uracil,
                Nucleotide::Cytosine => Nucleotide::Guanine,
                Nucleotide::Guanine => Nucleotide::Cytosine,
                Nucleotide::Thymine => Nucleotide::Adenine,
                _ => unreachable!(),
            })
            .collect();
        Rna(nucleotides)
    }

    const NUCLEOTIDES: [Nucleotide; 4] = [
        Nucleotide::Adenine,
        Nucleotide::Cytosine,
        Nucleotide::Guanine,
        Nucleotide::Thymine,
    ];
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        acid_from_string(rna, &Self::NUCLEOTIDES).map(Rna)
    }

    const NUCLEOTIDES: [Nucleotide; 4] = [
        Nucleotide::Adenine,
        Nucleotide::Cytosine,
        Nucleotide::Guanine,
        Nucleotide::Uracil,
    ];
}

fn acid_from_string(
    acid: &str,
    valid_nucleotides: &[Nucleotide],
) -> Result<Vec<Nucleotide>, usize> {
    acid.chars()
        .enumerate()
        .map(|(index, ch)| {
            nucleotide_translation(ch)
                .filter(|nucleotide| valid_nucleotides.contains(nucleotide))
                .ok_or(index)
        })
        .collect()
}

fn nucleotide_translation(ch: char) -> Option<Nucleotide> {
    match ch {
        'A' => Some(Nucleotide::Adenine),
        'C' => Some(Nucleotide::Cytosine),
        'G' => Some(Nucleotide::Guanine),
        'T' => Some(Nucleotide::Thymine),
        'U' => Some(Nucleotide::Uracil),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}
