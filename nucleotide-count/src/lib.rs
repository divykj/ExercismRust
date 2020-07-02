use std::collections::HashMap;

fn is_valid_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => return true,
        _ => return false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts: usize = 0;
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    for ch in dna.chars() {
        match ch {
            x if x == nucleotide => counts += 1,
            x if !is_valid_nucleotide(x) => return Err(x),
            _ => (),
        };
    }
    Ok(counts)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .iter()
        .cloned()
        .collect();

    for ch in dna.chars() {
        match ch {
            x if is_valid_nucleotide(x) => *counts.get_mut(&x).unwrap() += 1,
            x => return Err(x),
        };
    }
    Ok(counts)
}
