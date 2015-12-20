use std::fmt;
use super::RosalindError;
use super::RosalindError::UnknownNucleotide;

/// This structure contains amount of each nucleotid in DNA
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
/// let mut dna = "Z";
/// assert_eq!(count_dna_nucleotides(dna).unwrap_err(), UnknownNucleotide('Z'));
///
/// dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
/// let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
/// assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
/// assert_eq!(dna_nucleotides.to_string(), "20 12 17 21");
/// assert_eq!(count_dna_nucleotides("\n").unwrap(), DNANucleotides {A: 0, C: 0, G: 0, T: 0});
/// ```
pub fn count_dna_nucleotides(dna: &str) -> Result<DNANucleotides, RosalindError> {
  let mut dna_nucleotides = DNANucleotides {A: 0, C: 0, G: 0, T: 0};
  for nucleotid in dna.chars() {
    match nucleotid {
      'A' => dna_nucleotides.A += 1,
      'C' => dna_nucleotides.C += 1,
      'G' => dna_nucleotides.G += 1,
      'T' => dna_nucleotides.T += 1,
      '\n' => continue,
      _ => return Err(UnknownNucleotide(nucleotid))
    }
  }

  Ok(dna_nucleotides)
}
