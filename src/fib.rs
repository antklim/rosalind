//! Module for `Rabbits and Recurrence Relations`

/// This function calculates recurrence relation, or value of element at `n` position
/// in Fibonaccie's sequence
///
/// # Examples
/// ```
/// use rosalind::fib::*;
///
/// assert_eq!(recurrence_relation(5, 3), 19);
/// ```
#[allow(unused_variables)]
pub fn recurrence_relation(n: u8, k: u8) -> u64 {
  let mut pair: (u64, u64) = (1, 1);
  for i in 0..n - 1 {
    pair = (pair.1, pair.0 * k as u64 + pair.1)
  }
  return pair.0;
}

#[cfg(test)]
mod tests {
  use super::recurrence_relation;

  #[test]
  fn it_should_return_recurrence_relation() {
    assert_eq!(recurrence_relation(5, 3), 19);
  }
}
