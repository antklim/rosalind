//! Module for `Translating RNA into Protein`

use Result;
use RosalindError::{CodonParseError, UnknownCodon};

const CODON_STOP_SYMBOL: char = '\x00';

fn codon_into_amino_acid(codon: &str) -> Result<char> {
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
    "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG"=> Ok('L'),
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
    _ => Err(UnknownCodon(codon)),
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
/// assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ"));
/// ```
pub fn translate_rna_into_protein(rna: &str) -> Result<String> {
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

#[cfg(test)]
mod tests {
  use super::translate_rna_into_protein;
  use super::super::RosalindError::{CodonParseError, UnknownCodon};

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
    assert_eq!(translate_rna_into_protein("ZZZ").unwrap_err(), UnknownCodon("ZZZ"));
  }
}
