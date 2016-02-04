//! Module for `Complementing a Strand of DNA`

use Result;
use RosalindError::UnknownNucleotide;

/// This function returns a reverse complement of a DNA string
///
/// # Examples
/// ```
/// use rosalind::RosalindError::UnknownNucleotide;
/// use rosalind::revc::*;
///
/// let dna = "AAAACCCGGT";
/// assert_eq!(reverse_complement_dna(dna).unwrap(), "ACCGGGTTTT");
/// assert_eq!(reverse_complement_dna("\n").unwrap(), "");
/// assert_eq!(reverse_complement_dna("Z").unwrap_err(), UnknownNucleotide('Z'));
/// ```
pub fn reverse_complement_dna(dna: &str) -> Result<String> {
  let mut complement_dna = String::new();
  for nucleotide in dna.chars().rev() {
    match nucleotide {
      'A' => complement_dna.push('T'),
      'C' => complement_dna.push('G'),
      'G' => complement_dna.push('C'),
      'T' => complement_dna.push('A'),
      '\n' => continue,
      _ => return Err(UnknownNucleotide(nucleotide))
    }
  }

  Ok(complement_dna)
}

#[cfg(test)]
mod tests {
  use super::reverse_complement_dna;
  use super::super::RosalindError::UnknownNucleotide;

  #[test]
  fn it_should_return_error_when_unknown_nucleotide_found() {
    assert_eq!(reverse_complement_dna("Z").unwrap_err(), UnknownNucleotide('Z'));
  }

  #[test]
  fn it_should_reverse_complement_dna() {
    let dna = "AAAACCCGGT";
    assert_eq!(reverse_complement_dna(dna).unwrap(), "ACCGGGTTTT");
  }

  #[test]
  fn it_should_skip_new_line_symbol() {
    assert_eq!(reverse_complement_dna("\n").unwrap(), "");
  }
}
