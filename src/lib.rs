//! The `rosalind` crate provides fuctions to solve probles from [Rosalind](http://rosalind.info/) site.
//!
//! # Counting DNA Nucleotides
//! ## Examples
//! ```
//! use rosalind::RosalindError::UnknownNucleotide;
//! use rosalind::dna::*;
//!
//! let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
//! let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
//! assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
//! assert_eq!(dna_nucleotides.to_string(), "20 12 17 21");
//! assert_eq!(count_dna_nucleotides("\n").unwrap(), DNANucleotides {A: 0, C: 0, G: 0, T: 0});
//! assert_eq!(count_dna_nucleotides("Z").unwrap_err(), UnknownNucleotide('Z'));
//! ```
//!
//! # Transcribing DNA into RNA
//! ## Examples
//! ```
//! use rosalind::RosalindError::UnknownNucleotide;
//! use rosalind::rna::*;
//!
//! let dna = "GATGGAACTTGACTACGTAAATT";
//! assert_eq!(transcribe_dna_into_rna(dna).unwrap(), "GAUGGAACUUGACUACGUAAAUU");
//! assert_eq!(transcribe_dna_into_rna("\n").unwrap(), "");
//! assert_eq!(transcribe_dna_into_rna("Z").unwrap_err(), UnknownNucleotide('Z'));
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

pub mod dna;
pub mod rna;

#[cfg(test)]
mod tests {
  use super::RosalindError::UnknownNucleotide;
  use super::dna::*;
  use super::rna::*;

  // DNA =======================================================================
  #[test]
  fn dna_should_return_error_when_unknown_nucleotid_found() {
    assert_eq!(count_dna_nucleotides("Z").unwrap_err(), UnknownNucleotide('Z'));
  }

  #[test]
  fn dna_should_count_dna_nucleotides() {
    let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
    assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
  }

  #[test]
  fn dna_should_format_dna_nucleotides() {
    let dna_nucleotides = DNANucleotides {A: 1, C: 2, G: 3, T: 4};
    assert_eq!(dna_nucleotides.to_string(), "1 2 3 4");
  }

  #[test]
  fn dna_should_skip_new_line_symbol() {
    assert_eq!(count_dna_nucleotides("\n").unwrap(), DNANucleotides {A: 0, C: 0, G: 0, T: 0});
  }

  // RNA =======================================================================
  #[test]
  fn rna_should_return_error_when_unknown_nucleotid_found() {
    assert_eq!(transcribe_dna_into_rna("Z").unwrap_err(), UnknownNucleotide('Z'));
  }

  #[test]
  fn rna_should_transcribe_dna_into_rna() {
    let dna = "GATGGAACTTGACTACGTAAATT";
    assert_eq!(transcribe_dna_into_rna(dna).unwrap(), "GAUGGAACUUGACUACGUAAAUU");
  }

  #[test]
  fn rna_should_skip_new_line_symbol() {
    assert_eq!(transcribe_dna_into_rna("\n").unwrap(), "");
  }
}
