//! Module for `Rabbits and Recurrence Relations`, `Mortal Fibonacci Rabbits`

use RosalindResult;
use num::{BigUint, One};
use num::bigint::{ToBigUint};
use std::mem::replace;

/// This function calculates recurrence relation, or value of element at `n` position
/// in Fibonaccie's sequence
///
/// * _n_ months amount to calculate population
/// * _k_ offspring amount from each pair
///
/// # Examples
/// ```
/// # #[macro_use] extern crate num;
/// # #[macro_use] extern crate rosalind;
/// # fn main() {
/// use rosalind::fib::*;
/// use num::{BigUint};
/// use num::bigint::{ToBigUint};
///
/// let expected_relation: BigUint = 19.to_biguint().unwrap();
/// assert_eq!(recurrence_relation(5, 3).unwrap(), expected_relation);
/// # }
/// ```
pub fn recurrence_relation(n: usize, k: usize) -> RosalindResult<BigUint> {
  let mut p0: BigUint = One::one();
  let mut p1: BigUint = One::one();
  let big_k: BigUint = k.to_biguint().unwrap();

  for _ in 0..n - 1 {
    let p2 = p0 * &big_k + &p1;
    p0 = replace(&mut p1, p2);
  }

  Ok(p0)
}

/// This function calculates recurrence relation, or value of element at `n` position
/// in Fibonaccie's sequence with stop value which is lifetime of each pair
///
/// * _n_ months amount to calculate population
/// * _m_ lifetime in months
///
/// # Examples
/// # #[macro_use] extern crate num;
/// # #[macro_use] extern crate rosalind;
/// # fn main() {
/// use rosalind::fib::*;
/// use num::{BigUint};
/// use num::bigint::{ToBigUint};
///
/// let expected_relation: BigUint = 4.to_biguint().unwrap();
/// assert_eq!(recurrence_relation_with_stop(6, 3).unwrap(), expected_relation);
/// # }
/// ```
pub fn recurrence_relation_with_stop(n: usize, m: usize) -> RosalindResult<BigUint> {
  let mut f: BigUint;
  let vec_size: usize = m + 1 as usize;
  let mut d: Vec<BigUint> = Vec::with_capacity(vec_size);

  for i in 0..n {
    if i <= 1 {
      d.push(One::one());
    } else if i < m {
      {
        f = d.get(i - 2).unwrap() + d.get(i - 1).unwrap();
      }
      d.push(f);
    } else if i == m {
      {
        f = d.get(i - 2).unwrap() + d.get(i - 1).unwrap() - &(One::one());
      }
      d.push(f);
    } else {
      {
        f = d.get(m).unwrap() + d.get(m - 1).unwrap();
        f = f - d.remove(0);
      }
      d.push(f);
    }
  }

  Ok(d.pop().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;
  use num::{BigUint};
  use num::bigint::{ToBigUint};

  #[test]
  fn it_should_return_recurrence_relation() {
    let expected_relation: BigUint = 19.to_biguint().unwrap();
    assert_eq!(recurrence_relation(5, 3).unwrap(), expected_relation);
  }

  #[test]
  fn it_should_return_recurrence_relation_with_stop() {
    let expected_relation: BigUint = 4.to_biguint().unwrap();
    assert_eq!(recurrence_relation_with_stop(6, 3).unwrap(), expected_relation);
  }
}
