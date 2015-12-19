extern crate rosalind;

use rosalind::count_dna_nucleotides;

#[test]
fn it_should_count_dna_nucleotides() {
  let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
  assert_eq!("20 12 17 21", count_dna_nucleotides(dna));
}
