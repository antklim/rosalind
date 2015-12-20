extern crate rosalind;

use rosalind::RosalindError::UnknownNucleotid;
use rosalind::DNANucleotides;
use rosalind::count_dna_nucleotides;

#[test]
fn it_should_return_error_when_unknown_nucleotid_found() {
  let dna = "Z";
  assert_eq!(count_dna_nucleotides(dna).unwrap_err(), UnknownNucleotid('Z'));
}

#[test]
fn it_should_count_dna_nucleotides() {
  let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
  let dna_nucleotides = DNANucleotides {A: 20, C: 12, G: 17, T: 21};
  assert_eq!(count_dna_nucleotides(dna).unwrap(), dna_nucleotides);
}
