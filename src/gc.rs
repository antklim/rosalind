//! Module for `Computing GC Content`

use std::fmt;
use RosalindResult;
use constants::FASTA_LABEL_SYMBOL;

/// This structure contains info about the string with the highest GC content
#[allow(non_snake_case)]
#[derive(PartialEq, Debug)]
pub struct GCcontent {
  pub string_id: String,
  pub gc_content: f32
}

impl fmt::Display for GCcontent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\n{}", self.string_id, self.gc_content)
  }
}

/// This function calculates gc content of given DNA string
///
/// ## Examples
/// ```
/// use rosalind::gc::*;
///
/// assert_eq!(gc_content("").unwrap(), 0f32);
/// assert_eq!(gc_content("AGCTATAG").unwrap(), 37.5f32);
/// ```
pub fn gc_content(dna: &str) -> RosalindResult<f32> {
  let dna_len = dna.len();
  if dna_len == 0 { return Ok(0f32); }

  let gc_counter: f32 = dna.chars().fold(0f32, |mut counter, nucleotide| {
    match nucleotide {
      'G' | 'C' => counter += 1f32,
      _ => {},
    }
    counter
  });

  let gc: f32 = gc_counter * 100f32 / (dna_len as f32);
  Ok(gc)
}

/// This function calculates best gc content in the given dataset and
/// returns ID of the string with the highest GC content and GC content
///
/// ## Examples
/// ```
/// use rosalind::gc::*;
///
/// let dataset = ">Rosalind_6404
///   CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
///   TCCCACTAATAATTCTGAGG
///   >Rosalind_5959
///   CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
///   ATATCCATTTGTCAGCAGACACGC
///   >Rosalind_0808
///   CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
///   TGGGAACCTGCGGGCAGTAGGTGGAAT";
///
/// assert_eq!(best_gc_content_in_dataset(dataset).unwrap(),
///   GCcontent {string_id: "Rosalind_0808".to_string(), gc_content: 60.919540f32});
/// ```
// TODO: refactor - use utils:parse_fasta_dataset
pub fn best_gc_content_in_dataset(dataset: &str) -> RosalindResult<GCcontent> {
  let mut best_gc_label = "".to_string();
  let mut best_gc_content = 0f32;
  let mut current_gc_content: f32;
  let mut dna_string = "".to_string();

  for mut dataset_line in dataset.lines().rev() {
    dataset_line = dataset_line.trim();
    let first_symbol = dataset_line.chars().nth(0).unwrap();

    if first_symbol == FASTA_LABEL_SYMBOL {
      current_gc_content = gc_content(&dna_string).unwrap();

      if current_gc_content > best_gc_content {
        best_gc_label = dataset_line[1..].to_string();
        best_gc_content = current_gc_content;
      }

      dna_string = "".to_string();
    } else {
      dna_string = dna_string + dataset_line;
    }
  }

  Ok(GCcontent {string_id: best_gc_label, gc_content: best_gc_content})
}

#[cfg(test)]
mod tests {
  use super::gc_content;
  use super::best_gc_content_in_dataset;
  use super::GCcontent;

  #[test]
  fn it_should_return_0_for_empty_dna_string() {
    assert_eq!(gc_content("").unwrap(), 0f32);
  }

  #[test]
  fn it_should_calculate_gc_content_of_dna() {
    assert_eq!(gc_content("AGCTATAG").unwrap(), 37.5f32);
  }

  #[test]
  fn it_should_calculate_best_gc_content() {
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

  #[test]
  fn it_should_format_gc_content() {
    let gc_content = GCcontent {string_id: "Rosalind_0808".to_string(), gc_content: 123.45f32};
    assert_eq!(gc_content.to_string(), "Rosalind_0808\n123.45");
  }
}
