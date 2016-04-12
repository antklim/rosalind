//! Module for `Translating RNA into Protein, Inferring mRNA from Protein`

use RosalindResult;
use RosalindError::{CodonParseError, UnknownCodon, UnknownAminoAcid};

const CODON_STOP_SYMBOL: char = '\x00';

fn codon_into_amino_acid(codon: &str) -> RosalindResult<char> {
  match codon {
    "GCU" | "GCC" | "GCA" | "GCG" => Ok('A'),
    "UGU" | "UGC" => Ok('C'),
    "GAU" | "GAC" => Ok('D'),
    "GAA" | "GAG" => Ok('E'),
    "UUU" | "UUC" => Ok('F'),
    "GGU" | "GGC" | "GGA" | "GGG" => Ok('G'),
    "CAU" | "CAC" => Ok('H'),
    "AUU" | "AUC" | "AUA" => Ok('I'),
    "AAA" | "AAG" => Ok('K'),
    "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => Ok('L'),
    "AUG" => Ok('M'),
    "AAU" | "AAC" => Ok('N'),
    "CCU" | "CCC" | "CCA" | "CCG" => Ok('P'),
    "CAA" | "CAG" => Ok('Q'),
    "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => Ok('R'),
    "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => Ok('S'),
    "ACU" | "ACC" | "ACA" | "ACG" => Ok('T'),
    "GUU" | "GUC" | "GUA" | "GUG" => Ok('V'),
    "UGG" => Ok('W'),
    "UAU" | "UAC" => Ok('Y'),
    "UAA" | "UAG" | "UGA" => Ok(CODON_STOP_SYMBOL),
    _ => Err(UnknownCodon(codon.to_string())),
  }
}

fn amino_acid_into_codon<'a>(amino_acid: char) -> RosalindResult<Vec<&'a str>> {
  match amino_acid {
    'A' => Ok(vec!["GCU", "GCC", "GCA", "GCG"]),
    'C' => Ok(vec!["UGU", "UGC"]),
    'D' => Ok(vec!["GAU", "GAC"]),
    'E' => Ok(vec!["GAA", "GAG"]),
    'F' => Ok(vec!["UUU", "UUC"]),
    'G' => Ok(vec!["GGU", "GGC", "GGA", "GGG"]),
    'H' => Ok(vec!["CAU", "CAC"]),
    'I' => Ok(vec!["AUU", "AUC", "AUA"]),
    'K' => Ok(vec!["AAA", "AAG"]),
    'L' => Ok(vec!["UUA", "UUG", "CUU", "CUC", "CUA", "CUG"]),
    'M' => Ok(vec!["AUG"]),
    'N' => Ok(vec!["AAU", "AAC"]),
    'P' => Ok(vec!["CCU", "CCC", "CCA", "CCG"]),
    'Q' => Ok(vec!["CAA", "CAG"]),
    'R' => Ok(vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"]),
    'S' => Ok(vec!["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"]),
    'T' => Ok(vec!["ACU", "ACC", "ACA", "ACG"]),
    'V' => Ok(vec!["GUU", "GUC", "GUA", "GUG"]),
    'W' => Ok(vec!["UGG"]),
    'Y' => Ok(vec!["UAU", "UAC"]),
    CODON_STOP_SYMBOL => Ok(vec!["UAA", "UAG", "UGA"]),
    _ => Err(UnknownAminoAcid(amino_acid)),
  }
}

/// This function translates provided RNA string into protein string
///
/// ## Examples
/// ```
/// use rosalind::RosalindError::{CodonParseError, UnknownCodon};
/// use rosalind::prot::*;
///
/// let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
/// assert_eq!(translate_rna_into_protein(rna).unwrap(), "MAMAPRTEINSTRING");
/// assert_eq!(translate_rna_into_protein("AUGUGA\n").unwrap(), "M");
/// assert_eq!(translate_rna_into_protein("Z").unwrap_err(), CodonParseError);
/// assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ".to_string()));
/// ```
pub fn translate_rna_into_protein(rna: &str) -> RosalindResult<String> {
  let mut rna_len = rna.len();
  if rna.ends_with("\n") { rna_len = rna_len - 1; }
  if rna_len % 3 != 0 { return Err(CodonParseError); }

  let mut prot = String::new();
  for i in 0..(rna_len / 3) {
    let (left, right) = (i * 3, (i + 1) * 3);
    let amino_acid = try!(codon_into_amino_acid(&rna[left..right]));
    if amino_acid == CODON_STOP_SYMBOL { break; }
    prot.push(amino_acid);
  }

  Ok(prot)
}

/// This function returns the number of different mRNA strings from which protein
/// could have been translated
///
/// ## Examples
/// ```
/// use rosalind::RosalindError::UnknownAminoAcid;
/// use rosalind::prot::*;
///
/// assert_eq!(get_number_of_rna_from_protein("MA").unwrap(), 12);
/// assert_eq!(get_number_of_rna_from_protein("").unwrap(), 0);
/// assert_eq!(get_number_of_rna_from_protein("\n").unwrap(), 3);
/// assert_eq!(get_number_of_rna_from_protein("B").unwrap_err(), UnknownAminoAcid('B'));
/// ```
pub fn get_number_of_rna_from_protein(protein: &str) -> RosalindResult<usize> {
  if protein.len() == 0 { return Ok(0usize); }

  let mut total: usize = 1usize;
  let modulo: usize = 1000000usize;

  for amino_acid in protein.chars() {
    if amino_acid == '\n' { continue; }
    let possible_codons = try!(amino_acid_into_codon(amino_acid));
    total = total * possible_codons.len();
    if total > modulo { total = total % modulo }
    if total == 0usize { return Ok(0usize); }
  }

  let stop_codons = try!(amino_acid_into_codon(CODON_STOP_SYMBOL));
  total = total * stop_codons.len();

  Ok(total % modulo)
}

#[cfg(test)]
mod tests {
  use super::{translate_rna_into_protein, get_number_of_rna_from_protein};
  use super::super::RosalindError::{CodonParseError, UnknownCodon, UnknownAminoAcid};

  #[test]
  fn it_should_translate_rna_into_protein() {
    let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
    assert_eq!(translate_rna_into_protein(rna).unwrap(), "MAMAPRTEINSTRING");
  }

  #[test]
  fn it_should_ignore_new_line_symbol() {
    let rna = "AUGUGA\n";
    assert_eq!(translate_rna_into_protein(rna).unwrap(), "M");
  }

  #[test]
  fn it_should_return_error_when_cannot_parse_codons() {
    assert_eq!(translate_rna_into_protein("Z").unwrap_err(), CodonParseError);
  }

  #[test]
  fn it_should_return_error_when_unknown_codon_found() {
    assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ".to_string()));
  }

  #[test]
  fn it_should_return_number_of_rna_from_protein() {
    assert_eq!(get_number_of_rna_from_protein("MA").unwrap(), 12);
  }

  #[test]
  fn it_should_return_zero_for_empty_string() {
    assert_eq!(get_number_of_rna_from_protein("").unwrap(), 0);
  }

  #[test]
  fn it_should_return_amount_of_stop_codons_string() {
    assert_eq!(get_number_of_rna_from_protein("\n").unwrap(), 3);
  }

  #[test]
  fn it_should_return_error_when_unknown_aminoacid_found() {
    assert_eq!(get_number_of_rna_from_protein("B").unwrap_err(), UnknownAminoAcid('B'));
  }
}
