//! The `rosalind` crate provides fuctions to solve probles from [Rosalind](http://rosalind.info/) site.
//!
//! # Counting DNA Nucleotides
//! ## Examples
//! ```
//! use rosalind::RosalindError::UnknownNucleotide;
//! use rosalind::DNANucleotides;
//! use rosalind::count_dna_nucleotides;
//!
//! let mut dna = "Z";
//! assert_eq!(count_dna_nucleotides(dna).unwrap_err(), UnknownNucleotide('Z'));
//!
//! dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
//! let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
//! assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
//! assert_eq!(dna_nucleotides.to_string(), "20 12 17 21");
//! assert_eq!(count_dna_nucleotides("\n").unwrap(), DNANucleotides {A: 0, C: 0, G: 0, T: 0});
//! ```

use std::error::Error;
use std::fmt;

use self::RosalindError::{UnknownNucleotide};

#[derive(PartialEq, Debug)]
pub enum RosalindError {
  UnknownNucleotide(char),
}

impl fmt::Display for RosalindError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      UnknownNucleotide(ref nucleotid) => write!(f, "{}: '{}'", self.description(), nucleotid),
    }
  }
}

impl Error for RosalindError {
  fn description(&self) -> &str {
    match *self {
      UnknownNucleotide(..) => "Unknown nucleotid",
    }
  }
}

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
/// use rosalind::DNANucleotides;
/// use rosalind::count_dna_nucleotides;
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

#[cfg(test)]
mod tests {
  use super::RosalindError::UnknownNucleotide;
  use super::DNANucleotides;
  use super::count_dna_nucleotides;

  #[test]
  fn it_should_return_error_when_unknown_nucleotid_found() {
    let dna = "Z";
    assert_eq!(count_dna_nucleotides(dna).unwrap_err(), UnknownNucleotide('Z'));
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
