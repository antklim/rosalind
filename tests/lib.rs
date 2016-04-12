extern crate rosalind;
extern crate num;

use rosalind::RosalindError::*;
use rosalind::dna::*;
use rosalind::rna::*;
use rosalind::revc::*;
use rosalind::fib::*;
use rosalind::prot::*;
use rosalind::hamm::*;
use rosalind::subs::*;
use rosalind::gc::*;
use rosalind::iprb::*;

use num::{BigUint};
use num::bigint::{ToBigUint};

// DNA =========================================================================
#[test]
fn dna_should_return_error_when_unknown_nucleotide_found() {
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
fn rna_should_return_error_when_unknown_nucleotide_found() {
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
fn revc_should_return_error_when_unknown_nucleotide_found() {
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
  let expected_relation: BigUint = 19.to_biguint().unwrap();
  assert_eq!(recurrence_relation(5, 3).unwrap(), expected_relation);
}

#[test]
fn fib_should_return_recurrence_relation_with_stop() {
  let expected_relation: BigUint = 4.to_biguint().unwrap();
  assert_eq!(recurrence_relation_with_stop(6, 3).unwrap(), expected_relation);
}

// PROT ========================================================================
#[test]
fn prot_should_translate_rna_into_protein() {
  let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
  assert_eq!(translate_rna_into_protein(rna).unwrap(), "MAMAPRTEINSTRING");
}

#[test]
fn prot_should_ignore_new_line_symbol() {
  let rna = "AUGUGA\n";
  assert_eq!(translate_rna_into_protein(rna).unwrap(), "M");
}

#[test]
fn prot_should_return_error_when_cannot_parse_codons() {
  assert_eq!(translate_rna_into_protein("Z").unwrap_err(), CodonParseError);
}

#[test]
fn prot_should_return_error_when_unknown_codon_found() {
  assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ".to_string()));
}

#[test]
fn prot_should_return_number_of_rna_from_protein() {
  assert_eq!(get_number_of_rna_from_protein("MA").unwrap(), 12);
}

#[test]
fn prot_should_return_zero_for_empty_string() {
  assert_eq!(get_number_of_rna_from_protein("").unwrap(), 0);
}

#[test]
fn prot_should_return_amount_of_stop_codons_string() {
  assert_eq!(get_number_of_rna_from_protein("\n").unwrap(), 3);
}

#[test]
fn prot_should_return_error_when_unknown_aminoacid_found() {
  assert_eq!(get_number_of_rna_from_protein("B").unwrap_err(), UnknownAminoAcid('B'));
}

// HAMM ========================================================================
#[test]
fn hamm_should_return_hamming_distance() {
  let s = "GAGCCTACTAACGGGAT";
  let t = "CATCGTAATGACGGCCT";
  assert_eq!(hamming_distance(s, t).unwrap(), 7);
}

#[test]
fn hamm_should_return_error_when_strings_have_different_length() {
  assert_eq!(hamming_distance("G", "").unwrap_err(), HammingStringsLengthError);
}

// SUBS ========================================================================
#[test]
fn subs_should_return_all_locations_of_substring_t_in_s() {
  let s = "GATATATGCATATACTT";
  let t = "ATAT";
  assert_eq!(motif_lookup(s, t).unwrap(), vec![2, 4, 10]);
}

#[test]
fn subs_should_return_error_when_substring_t_is_longer_than_s() {
  let s = "ATAT";
  let t = "GATATATGCATATACTT";
  assert_eq!(motif_lookup(s, t).unwrap_err(), MotifStringsLengthError);
}

// GC ==========================================================================
#[test]
fn gc_should_return_0_for_empty_dna_string() {
  assert_eq!(gc_content("").unwrap(), 0f32);
}

#[test]
fn gc_should_calculate_gc_content_of_dna() {
  assert_eq!(gc_content("AGCTATAG").unwrap(), 37.5f32);
}

#[test]
fn gc_should_calculate_best_gc_content() {
  let dataset = ">Rosalind_6404
    CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
    TCCCACTAATAATTCTGAGG
    >Rosalind_5959
    CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
    ATATCCATTTGTCAGCAGACACGC
    >Rosalind_0808
    CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
    TGGGAACCTGCGGGCAGTAGGTGGAAT";

  assert_eq!(best_gc_content_in_dataset(dataset).unwrap(),
    GCcontent {string_id: "Rosalind_0808".to_string(), gc_content: 60.919540f32});
}

// IPRB ========================================================================
#[test]
fn iprb_should_retutn_error_when_invalid_input_parameters_provided() {
  assert_eq!(dominant_allele_probability(0, 1, 1).unwrap_err(), InvalidInputParameters);
  assert_eq!(dominant_allele_probability(1, 0, 1).unwrap_err(), InvalidInputParameters);
  assert_eq!(dominant_allele_probability(1, 1, 0).unwrap_err(), InvalidInputParameters);
}

#[test]
fn iprb_should_retutn_dominant_allele_probability() {
  assert_eq!(dominant_allele_probability(2, 2, 2).unwrap(), 0.7833333);
}
