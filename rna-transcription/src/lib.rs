#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    fn is_valid_dna_nucleotide(nucleotide: char) -> bool {
        match nucleotide {
            'A' | 'C' | 'G' | 'T' => return true,
            _ => return false,
        }
    }

    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, nucleotide) in dna.chars().enumerate() {
            if !DNA::is_valid_dna_nucleotide(nucleotide) {
                return Err(i);
            }
        }
        Ok(DNA {
            strand: String::from(dna),
        })
    }

    pub fn into_rna(self) -> RNA {
        return RNA::new(
            &self
                .strand
                .chars()
                .map(|nucleotide| match nucleotide {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => ' ',
                })
                .collect::<String>(),
        )
        .unwrap();
    }
}

impl RNA {
    fn is_valid_rna_nucleotide(nucleotide: char) -> bool {
        match nucleotide {
            'A' | 'C' | 'G' | 'U' => return true,
            _ => return false,
        }
    }

    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, nucleotide) in rna.chars().enumerate() {
            if !RNA::is_valid_rna_nucleotide(nucleotide) {
                return Err(i);
            }
        }
        Ok(RNA {
            strand: String::from(rna),
        })
    }
}
