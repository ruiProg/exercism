use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)
        .and_then(|dna_nucleotides| dna_nucleotides.get(&nucleotide).ok_or(nucleotide).copied())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_nucleotides: HashMap<char, usize> = ['A', 'T', 'C', 'G']
        .map(|nucleotide| (nucleotide, 0))
        .into();

    for ch in dna.chars() {
        let nucleotide_count = dna_nucleotides.get_mut(&ch).ok_or(ch)?;
        *nucleotide_count += 1;
    }

    Ok(dna_nucleotides)
}
