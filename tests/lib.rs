extern crate rosalind;

use rosalind::RosalindError::UnknownNucleotide;
use rosalind::dna::*;

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
