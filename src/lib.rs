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
//! # #[macro_use] extern crate num;
//! # #[macro_use] extern crate rosalind;
//! # fn main() {
//! use rosalind::fib::*;
//! use num::{BigUint};
//! use num::bigint::{ToBigUint};
//!
//! let mut expected_relation: BigUint = 19.to_biguint().unwrap();
//! assert_eq!(recurrence_relation(5, 3).unwrap(), expected_relation);
//! expected_relation = 4.to_biguint().unwrap();
//! assert_eq!(recurrence_relation_with_stop(6, 3).unwrap(), expected_relation);
//! # }
//! ```
//!
//! # Translating RNA into Protein, Inferring mRNA from Protein
//! ## Examples
//! ```
//! use rosalind::RosalindError::{CodonParseError, UnknownCodon, UnknownAminoAcid};
//! use rosalind::prot::*;
//!
//! let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
//! assert_eq!(translate_rna_into_protein(rna).unwrap(), "MAMAPRTEINSTRING");
//! assert_eq!(translate_rna_into_protein("AUGUGA\n").unwrap(), "M");
//! assert_eq!(translate_rna_into_protein("Z").unwrap_err(), CodonParseError);
//! assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ".to_string()));
//!
//! assert_eq!(get_number_of_rna_from_protein("MA").unwrap(), 12);
//! assert_eq!(get_number_of_rna_from_protein("").unwrap(), 0);
//! assert_eq!(get_number_of_rna_from_protein("\n").unwrap(), 3);
//! assert_eq!(get_number_of_rna_from_protein("B").unwrap_err(), UnknownAminoAcid('B'));
//! ```
//!
//! # Counting Point Mutations
//! ## Examples
//! ```
//! use rosalind::RosalindError::HammingStringsLengthError;
//! use rosalind::hamm::*;
//!
//! let s = "GAGCCTACTAACGGGAT";
//! let t = "CATCGTAATGACGGCCT";
//! assert_eq!(hamming_distance(s, t).unwrap(), 7);
//! assert_eq!(hamming_distance("G", "").unwrap_err(), HammingStringsLengthError);
//! ```
//!
//! # Finding a Motif in DNA
//! ## Examples
//! ```
//! use rosalind::RosalindError::MotifStringsLengthError;
//! use rosalind::subs::*;
//!
//! let s = "GATATATGCATATACTT";
//! let t = "ATAT";
//! assert_eq!(motif_lookup(s, t).unwrap(), vec![2, 4, 10]);
//! assert_eq!(motif_lookup(t, s).unwrap_err(), MotifStringsLengthError);
//! ```
//!
//! # Computing GC Content
//! ## Examples
//! ```
//! use rosalind::gc::*;
//!
//! assert_eq!(gc_content("").unwrap(), 0f32);
//! assert_eq!(gc_content("AGCTATAG").unwrap(), 37.5f32);
//!
//! let dataset = ">Rosalind_6404
//!   CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
//!   TCCCACTAATAATTCTGAGG
//!   >Rosalind_5959
//!   CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
//!   ATATCCATTTGTCAGCAGACACGC
//!   >Rosalind_0808
//!   CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
//!   TGGGAACCTGCGGGCAGTAGGTGGAAT";
//!
//! assert_eq!(best_gc_content_in_dataset(dataset).unwrap(),
//!   GCcontent {string_id: "Rosalind_0808".to_string(), gc_content: 60.919540f32});
//! ```
//!
//! # Mendel's First Law
//! ## Examples
//! ```
//! use rosalind::RosalindError::InvalidInputParameters;
//! use rosalind::iprb::*;
//!
//! assert_eq!(dominant_allele_probability(0, 1, 1).unwrap_err(), InvalidInputParameters);
//! assert_eq!(dominant_allele_probability(1, 0, 1).unwrap_err(), InvalidInputParameters);
//! assert_eq!(dominant_allele_probability(1, 1, 0).unwrap_err(), InvalidInputParameters);
//!
//! assert_eq!(dominant_allele_probability(2, 2, 2).unwrap(), 0.7833333);
//! ```
extern crate num;

use std::error::Error;
use std::fmt;
use std::result;

use self::RosalindError::*;

#[derive(PartialEq, Debug)]
pub enum RosalindError {
  UnknownNucleotide(char),
  UnknownCodon(String),
  UnknownAminoAcid(char),
  CodonParseError,
  HammingStringsLengthError,
  MotifStringsLengthError,
  InvalidInputParameters,
}

impl fmt::Display for RosalindError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      UnknownNucleotide(ref nucleotide) => write!(f, "{}: '{}'", self.description(), nucleotide),
      UnknownCodon(ref codon) => write!(f, "{}: '{}'", self.description(), codon),
      UnknownAminoAcid(ref amino_acid) => write!(f, "{}: '{}'", self.description(), amino_acid),
      _ => write!(f, "{}", self.description()),
    }
  }
}

impl Error for RosalindError {
  fn description(&self) -> &str {
    match *self {
      UnknownNucleotide(..) => "Unknown nucleotide",
      UnknownCodon(..) => "Unknown codon",
      UnknownAminoAcid(..) => "Unknown amino acid",
      CodonParseError => "Could not parse RNA string and group codons",
      HammingStringsLengthError => "Strings must have equal length",
      MotifStringsLengthError => "Substrig `t` must be no longer than `s`",
      InvalidInputParameters => "Invalid input parameters have been passed to the function"
    }
  }
}

/// Unified return type for all modules and methods of `rosalind` library
///
/// ## Examples
/// ```
/// use rosalind::RosalindResult;
/// use rosalind::RosalindError::UnknownNucleotide;
/// use rosalind::dna::count_dna_nucleotides;
/// use rosalind::rna::transcribe_dna_into_rna;
///
/// fn wrapper<T, U>(method: &Fn(U) -> RosalindResult<T>, args: U) -> RosalindResult<T> {
///   method(args)
/// }
///
/// let result = wrapper(&transcribe_dna_into_rna, "GATGGAACTTGACTACGTAAATT");
/// assert_eq!(result.unwrap(), "GAUGGAACUUGACUACGUAAAUU");
///
/// let result = wrapper(&count_dna_nucleotides, "Z");
/// assert_eq!(result.unwrap_err(), UnknownNucleotide('Z'));
/// ```
pub type RosalindResult<T> = result::Result<T, RosalindError>;

pub mod dna;
pub mod rna;
pub mod revc;
pub mod fib;
pub mod prot;
pub mod hamm;
pub mod subs;
pub mod gc;
pub mod iprb;

#[cfg(test)]
mod tests {
  use super::RosalindError;
  use super::RosalindError::CodonParseError;

  #[test]
  fn it_should_stringify_rosalind_error() {
    let error: RosalindError = CodonParseError;
    assert_eq!(error.to_string(), "Could not parse RNA string and group codons");
  }
}
