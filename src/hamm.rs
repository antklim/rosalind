//! Module for `Counting Point Mutations`

use Result;
use RosalindError::HammingStringsLengthError;

/// This function calculates Hamming distance between `s` and `t`
///
/// # Examples
/// ```
/// use rosalind::RosalindError::HammingStringsLengthError;
/// use rosalind::hamm::*;
///
/// let s = "GAGCCTACTAACGGGAT";
/// let t = "CATCGTAATGACGGCCT";
/// assert_eq!(hamming_distance(s, t).unwrap(), 7);
/// assert_eq!(hamming_distance("G", "").unwrap_err(), HammingStringsLengthError);
/// ```
pub fn hamming_distance<'a>(s: &str, t: &str) -> Result<'a, u16> {
  if s.len() != t.len() { return Err(HammingStringsLengthError); }

  let s_iter = s.chars();
  let t_iter = t.chars();

  let distance: u16 = s_iter.zip(t_iter).fold(0, |mut d, (sc, tc)| {
    if sc != tc { d += 1; }
    d
  });

  Ok(distance)
}

#[cfg(test)]
mod tests {
  use super::hamming_distance;
  use super::super::RosalindError::HammingStringsLengthError;

  #[test]
  fn it_should_return_hamming_distance() {
    let s = "GAGCCTACTAACGGGAT";
    let t = "CATCGTAATGACGGCCT";
    assert_eq!(hamming_distance(s, t).unwrap(), 7);
  }

  #[test]
  fn it_should_return_error_when_strings_have_different_length() {
    assert_eq!(hamming_distance("G", "").unwrap_err(), HammingStringsLengthError);
  }
}
