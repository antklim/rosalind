//! Module for `Counting DNA Nucleotides`

use std::fmt;
use Result;
use RosalindError::UnknownNucleotide;

/// This structure contains amount of each nucleotide in DNA
#[allow(non_snake_case)]
#[derive(PartialEq, Debug)]
pub struct DNANucleotides {
  pub A: u32,
  pub C: u32,
  pub G: u32,
  pub T: u32
}

impl fmt::Display for DNANucleotides {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {} {}", self.A, self.C, self.G, self.T)
  }
}

/// This function calculates dna nucleotides
///
/// # Examples
/// ```
/// use rosalind::RosalindError::UnknownNucleotide;
/// use rosalind::dna::*;
///
/// let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
/// let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
/// assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
/// assert_eq!(dna_nucleotides.to_string(), "20 12 17 21");
/// assert_eq!(count_dna_nucleotides("\n").unwrap(), DNANucleotides {A: 0, C: 0, G: 0, T: 0});
/// assert_eq!(count_dna_nucleotides("Z").unwrap_err(), UnknownNucleotide('Z'));
/// ```
pub fn count_dna_nucleotides(dna: &str) -> Result<DNANucleotides> {
  let mut dna_nucleotides = DNANucleotides {A: 0, C: 0, G: 0, T: 0};
  for nucleotide in dna.chars() {
    match nucleotide {
      'A' => dna_nucleotides.A += 1,
      'C' => dna_nucleotides.C += 1,
      'G' => dna_nucleotides.G += 1,
      'T' => dna_nucleotides.T += 1,
      '\n' => continue,
      _ => return Err(UnknownNucleotide(nucleotide))
    }
  }

  Ok(dna_nucleotides)
}

#[cfg(test)]
mod tests {
  use super::DNANucleotides;
  use super::count_dna_nucleotides;
  use super::super::RosalindError::UnknownNucleotide;

  #[test]
  fn it_should_return_error_when_unknown_nucleotide_found() {
    assert_eq!(count_dna_nucleotides("Z").unwrap_err(), UnknownNucleotide('Z'));
  }

  #[test]
  fn it_should_count_dna_nucleotides() {
    let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
    assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
  }

  #[test]
  fn it_should_format_dna_nucleotides() {
    let dna_nucleotides = DNANucleotides {A: 1, C: 2, G: 3, T: 4};
    assert_eq!(dna_nucleotides.to_string(), "1 2 3 4");
  }

  #[test]
  fn it_should_skip_new_line_symbol() {
    assert_eq!(count_dna_nucleotides("\n").unwrap(), DNANucleotides {A: 0, C: 0, G: 0, T: 0});
  }
}
