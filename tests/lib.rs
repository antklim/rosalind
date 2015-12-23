extern crate rosalind;

use rosalind::RosalindError::UnknownNucleotide;
use rosalind::dna::*;
use rosalind::rna::*;
use rosalind::revc::*;
use rosalind::fib::*;

// DNA =========================================================================
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

// RNA =========================================================================
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

// REVC ========================================================================
#[test]
fn revc_should_return_error_when_unknown_nucleotid_found() {
  assert_eq!(reverse_complement_dna("Z").unwrap_err(), UnknownNucleotide('Z'));
}

#[test]
fn revc_should_reverse_complement_dna() {
  let dna = "AAAACCCGGT";
  assert_eq!(reverse_complement_dna(dna).unwrap(), "ACCGGGTTTT");
}

#[test]
fn revc_should_skip_new_line_symbol() {
  assert_eq!(reverse_complement_dna("\n").unwrap(), "");
}

// FIB =========================================================================
#[test]
fn fib_should_return_recurrence_relation() {
  assert_eq!(recurrence_relation(5, 3), 19);
}

