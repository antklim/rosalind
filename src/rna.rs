//! Module for `Transcribing DNA into RNA`

use RosalindError;
use RosalindError::UnknownNucleotide;

/// This function transcribes DNA into RNA via replacung T nucleotide in DNA
/// into U nucleotide in RNA
///
/// # Examples
/// ```
/// use rosalind::RosalindError::UnknownNucleotide;
/// use rosalind::rna::*;
///
/// let dna = "GATGGAACTTGACTACGTAAATT";
/// assert_eq!(transcribe_dna_into_rna(dna).unwrap(), "GAUGGAACUUGACUACGUAAAUU");
/// assert_eq!(transcribe_dna_into_rna("\n").unwrap(), "");
/// assert_eq!(transcribe_dna_into_rna("Z").unwrap_err(), UnknownNucleotide('Z'));
/// ```
pub fn transcribe_dna_into_rna(dna: &str) -> Result<String, RosalindError> {
  let mut rna = String::new();
  for nucleotide in dna.chars() {
    match nucleotide {
        'A' | 'C' | 'G' => rna.push(nucleotide),
        'T' => rna.push('U'),
        '\n' => continue,
        _ => return Err(UnknownNucleotide(nucleotide)),
    }
  }

  Ok(rna)
}

#[cfg(test)]
mod tests {
  use super::transcribe_dna_into_rna;
  use super::super::RosalindError::UnknownNucleotide;

  #[test]
  fn it_should_return_error_when_unknown_nucleotid_found() {
    assert_eq!(transcribe_dna_into_rna("Z").unwrap_err(), UnknownNucleotide('Z'));
  }

  #[test]
  fn it_should_transcribe_dna_into_rna() {
    let dna = "GATGGAACTTGACTACGTAAATT";
    assert_eq!(transcribe_dna_into_rna(dna).unwrap(), "GAUGGAACUUGACUACGUAAAUU");
  }

  #[test]
  fn it_should_skip_new_line_symbol() {
    assert_eq!(transcribe_dna_into_rna("\n").unwrap(), "");
  }
}
