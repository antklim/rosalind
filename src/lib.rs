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
//!
//! # Complementing a Strand of DNA
//! ## Examples
//! ```
//! use rosalind::RosalindError::UnknownNucleotide;
//! use rosalind::revc::*;
//!
//! let dna = "AAAACCCGGT";
//! assert_eq!(reverse_complement_dna(dna).unwrap(), "ACCGGGTTTT");
//! assert_eq!(reverse_complement_dna("\n").unwrap(), "");
//! assert_eq!(reverse_complement_dna("Z").unwrap_err(), UnknownNucleotide('Z'));
//! ```
//!
//! # Rabbits and Recurrence Relations
//! ## Examples
//! ```
//! use rosalind::fib::*;
//!
//! assert_eq!(recurrence_relation(5, 3), 19);
//! ```
//! # Translating RNA into Protein
//! ## Examples
//! ```
//! use rosalind::RosalindError::{CodonParseError, UnknownCodon};
//! use rosalind::prot::*;
//!
//! let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
//! assert_eq!(translate_rna_into_protein(rna).unwrap(), "MAMAPRTEINSTRING");
//! assert_eq!(translate_rna_into_protein("Z").unwrap_err(), CodonParseError);
//! assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ"));
//! ```

use std::error::Error;
use std::fmt;

use self::RosalindError::{UnknownNucleotide, CodonParseError, UnknownCodon};

#[derive(PartialEq, Debug)]
pub enum RosalindError<'a> {
  UnknownNucleotide(char),
  UnknownCodon(&'a str),
  CodonParseError,
}

impl<'a> fmt::Display for RosalindError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      UnknownNucleotide(ref nucleotide) => write!(f, "{}: '{}'", self.description(), nucleotide),
      UnknownCodon(ref codon) => write!(f, "{}: '{}'", self.description(), codon),
      _ => write!(f, "{}", self.description()),
    }
  }
}

impl<'a> Error for RosalindError<'a> {
  fn description(&self) -> &str {
    match *self {
      UnknownNucleotide(..) => "Unknown nucleotide",
      UnknownCodon(..) => "Unknown codon",
      CodonParseError => "Could not parse RNA string and group codons",
    }
  }
}

pub mod dna;
pub mod rna;
pub mod revc;
pub mod fib;
pub mod prot;
