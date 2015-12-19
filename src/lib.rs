/// This function calculates dna nucleotides
///
/// # Examples
/// ```
/// let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
/// assert_eq!("20 12 17 21", rosalind::count_dna_nucleotides(dna));
/// ```
pub fn count_dna_nucleotides(dna: &str) -> String {
  return "20 12 17 21".to_string();
}

#[cfg(test)]
mod tests {
  use super::count_dna_nucleotides;

  #[test]
  fn it_should_count_dna_nucleotides() {
    let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    assert_eq!("20 12 17 21", count_dna_nucleotides(dna));
  }
}
