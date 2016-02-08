//! Module for `Finding a Motif in DNA`

use Result;
use RosalindError::MotifStringsLengthError;


/// This function finds locations of substring `t` in string `s` (finds a motif in DNA)
///
/// ```
/// use rosalind::RosalindError::MotifStringsLengthError;
/// use rosalind::subs::*;
///
/// let s = "GATATATGCATATACTT";
/// let t = "ATAT";
/// assert_eq!(motif_lookup(s, t).unwrap(), vec![2, 4, 10]);
/// assert_eq!(motif_lookup(t, s).unwrap_err(), MotifStringsLengthError);
/// ```
pub fn motif_lookup<'a>(s: &str, t: &str) -> Result<'a, Vec<usize>> {
  let (s_len, t_len) = (s.len(), t.len());
  if s_len < t_len { return Err(MotifStringsLengthError); }
  let mut motif: Vec<usize> = Vec::new();
  for i in 0..(s_len - t_len + 1) {
    if s[i..].starts_with(t) { motif.push(i + 1); }
  }
  Ok(motif)
}

#[cfg(test)]
mod tests {
  use super::motif_lookup;
  use super::super::RosalindError::MotifStringsLengthError;

  #[test]
  fn it_should_return_all_locations_of_substring_t_in_s() {
    let s = "GATATATGCATATACTT";
    let t = "ATAT";
    assert_eq!(motif_lookup(s, t).unwrap(), vec![2, 4, 10]);
  }

  #[test]
  fn it_should_return_error_when_substring_t_is_longer_than_s() {
    let s = "ATAT";
    let t = "GATATATGCATATACTT";
    assert_eq!(motif_lookup(s, t).unwrap_err(), MotifStringsLengthError);
  }
}
