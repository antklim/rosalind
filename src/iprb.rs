//! Module for `Mendel's First Law`

use RosalindResult;
use RosalindError::InvalidInputParameters;

/// This function returns the probability that two randomly selected mating
/// organisms will produce an individual possessing a dominant allele.
///
/// * _k_ individuals are homozygous dominant for a factor
/// * _m_ individuals are heterozygous
/// * _n_ individuals are homozygous recessive
///
/// # Examples
/// ```
/// use rosalind::RosalindError::InvalidInputParameters;
/// use rosalind::iprb::*;
///
/// assert_eq!(dominant_allele_probability(0, 1, 1).unwrap_err(), InvalidInputParameters);
/// assert_eq!(dominant_allele_probability(1, 0, 1).unwrap_err(), InvalidInputParameters);
/// assert_eq!(dominant_allele_probability(1, 1, 0).unwrap_err(), InvalidInputParameters);
///
/// assert_eq!(dominant_allele_probability(2, 2, 2).unwrap(), 0.7833333);
/// ```
pub fn dominant_allele_probability(k: u8, m: u8, n: u8) -> RosalindResult<f32> {
  if k <= 0 || m <= 0 || n <= 0 { return Err(InvalidInputParameters); }
  let k_f32: f32 = k as f32;
  let m_f32: f32 = m as f32;
  let n_f32: f32 = n as f32;
  let total: f32 = k_f32 + m_f32 + n_f32;

  // calculate probability of recessive allele
  let mut p_recessive: f32 = 0.0f32;
  p_recessive += m_f32 * n_f32;                      // `Aa` and `aa`, `aa` and `Aa`
  p_recessive += m_f32 * (m_f32 - 1.0f32) * 0.25f32; // `Aa` and `Aa`
  p_recessive += n_f32 * (n_f32 - 1.0f32);           // `aa` and `aa`
  p_recessive = p_recessive / (total * (total - 1.0f32));

  Ok(1.0f32 - p_recessive)
}

#[cfg(test)]
mod tests {
  use super::dominant_allele_probability;
  use super::super::RosalindError::InvalidInputParameters;

  #[test]
  fn it_should_retutn_error_when_invalid_input_parameters_provided() {
    assert_eq!(dominant_allele_probability(0, 1, 1).unwrap_err(), InvalidInputParameters);
    assert_eq!(dominant_allele_probability(1, 0, 1).unwrap_err(), InvalidInputParameters);
    assert_eq!(dominant_allele_probability(1, 1, 0).unwrap_err(), InvalidInputParameters);
  }

  #[test]
  fn it_should_retutn_dominant_allele_probability() {
    assert_eq!(dominant_allele_probability(2, 2, 2).unwrap(), 0.7833333);
  }
}
