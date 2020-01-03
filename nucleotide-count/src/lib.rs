use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_nuleotide = is_valid_nucleotide(&nucleotide);
    if !valid_nuleotide {
        return Err(nucleotide);
    }
    dna.chars()
        .collect::<Vec<char>>()
        .iter()
        .fold(Ok(0), |acc, n| match acc {
            Ok(count) => {
                if is_valid_nucleotide(n) {
                    Ok(count + if *n == nucleotide { 1 } else { 0 })
                } else {
                    Err(*n)
                }
            }
            Err(err) => Err(err),
        })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut all_count = HashMap::new();
    for n in vec!['A', 'C', 'G', 'T'] {
        let count = count(n, dna)?;
        all_count.insert(n, count);
    }
    Ok(all_count)
}

fn is_valid_nucleotide(nucleotide: &char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}
