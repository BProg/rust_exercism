#[derive(Debug, PartialEq)]
pub struct DNA {
  dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
  rna: String,
}

impl DNA {
  pub fn new(dna: &str) -> Result<DNA, usize> {
    let verified_dna =
      dna
        .chars()
        .enumerate()
        .fold(Ok(String::new()), |acc, (index, nucleotide)| match acc {
          Ok(mut acc) => {
            if is_invalid_nucleotide(nucleotide) {
              return Err(index);
            } else {
              acc.push(nucleotide);
              return Ok(acc);
            }
          }
          Err(index) => Err(index),
        });
    verified_dna.map(|dna| DNA { dna })
  }

  pub fn into_rna(self) -> RNA {
    RNA {
      rna: self
        .dna
        .chars()
        .map(|c: char| match c {
          'G' => 'C',
          'C' => 'G',
          'T' => 'A',
          'A' => 'U',
          _ => ' ',
        })
        .fold(String::new(), |mut acc, c| {
          acc.push(c);
          acc
        }),
    }
  }
}

impl RNA {
  pub fn new(rna: &str) -> Result<RNA, usize> {
    let verified_rna =
      rna
        .chars()
        .enumerate()
        .fold(Ok(String::new()), |acc, (index, nucleotide)| match acc {
          Ok(mut acc) => {
            if is_invalid_nucleotide_rna(nucleotide) {
              return Err(index);
            } else {
              acc.push(nucleotide);
              return Ok(acc);
            }
          }
          Err(index) => Err(index),
        });
    verified_rna.map(|rna| RNA { rna })
  }
}

fn is_valid_nucleotide(nucleotide: char) -> bool {
  match nucleotide {
    'A' | 'C' | 'G' | 'T' => true,
    _ => false,
  }
}

fn is_invalid_nucleotide(n: char) -> bool {
  !is_valid_nucleotide(n)
}

fn is_valid_nucleotide_rna(nucleotide: char) -> bool {
  match nucleotide {
    'A' | 'C' | 'G' | 'U' => true,
    _ => false,
  }
}
fn is_invalid_nucleotide_rna(n: char) -> bool {
  !is_valid_nucleotide_rna(n)
}
