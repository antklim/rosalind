use super::RosalindError;
use super::RosalindError::UnknownNucleotide;

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
