// Positions are matching their DNA -> RNA transcription, DNA_NUCLEOTIDES[i] == RNA_NUCLEOTIDES[i]
const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq)]
pub struct DNA {
  strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
  strand: String,
}

impl DNA {
  pub fn new(dna: &str) -> Result<DNA, usize> {
    check_for_errors(dna, DNA_NUCLEOTIDES).map(|strand| DNA { strand })
  }

  pub fn into_rna(self) -> RNA {
    RNA {
      strand: self
        .strand
        .chars()
        .map(|c: char| {
          RNA_NUCLEOTIDES[DNA_NUCLEOTIDES
            .iter()
            .position(|&n| n == c)
            .expect("Bad DNA")]
        })
        .collect(),
    }
  }
}

impl RNA {
  pub fn new(rna: &str) -> Result<RNA, usize> {
    check_for_errors(rna, RNA_NUCLEOTIDES).map(|strand| RNA { strand })
  }
}

fn is_part_of(c: char, set: [char; 4]) -> bool {
  (&set).iter().any(|n| *n == c)
}

fn check_for_errors(source: &str, validation_set: [char; 4]) -> Result<String, usize> {
  let error = source
    .char_indices()
    .find(|(_, ch)| !is_part_of(*ch, validation_set));
  match error {
    Some((index, _)) => Err(index),
    None => Ok(String::from(source)),
  }
}
