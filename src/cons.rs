//! Module for `Consensus and Profile`

use std::fmt;
use RosalindResult;
use RosalindError::UnknownNucleotide;

/// This structure contains profile of DNA strings
#[allow(non_snake_case)]
#[derive(PartialEq, Debug)]
pub struct Profile {
    pub A: Vec<u32>,
    pub C: Vec<u32>,
    pub G: Vec<u32>,
    pub T: Vec<u32>,
}

impl fmt::Display for Profile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "A: {:?}\nC: {:?}\nG: {:?}\nT: {:?}", self.A, self.C, self.G, self.T)
  }
}

/// This function calculates profile by given DNA strings
///
/// ## Examples
/// ```
/// use rosalind::cons::*;
///
/// let dna_list = vec![
///     "ATCCAGCT",
///     "GGGCAACT",
///     "ATGGATCT",
///     "AAGCAACC",
///     "TTGGAACT",
///     "ATGCCATT",
///     "ATGGCACT",
/// ];
///
/// let expected_profile = Profile {
///     A: vec![5, 1, 0, 0, 5, 5, 0, 0],
///     C: vec![0, 0, 1, 4, 2, 0, 6, 1],
///     G: vec![1, 1, 6, 3, 0, 1, 0, 0],
///     T: vec![1, 5, 0, 0, 0, 1, 1, 6],
/// };
///
/// assert_eq!(profile(dna_list).unwrap(), expected_profile);
/// ```
pub fn profile(dna_list: Vec<&str>) -> RosalindResult<Profile> {
    let buf_capacity = dna_list[0].len();
    let mut profile = Profile {
        A: vec![0; buf_capacity],
        C: vec![0; buf_capacity],
        G: vec![0; buf_capacity],
        T: vec![0; buf_capacity],
    };

    for dna in dna_list.iter() {
        for (i, nucleotide) in dna.chars().enumerate() {
            match nucleotide {
                'A' => profile.A[i] += 1,
                'C' => profile.C[i] += 1,
                'G' => profile.G[i] += 1,
                'T' => profile.T[i] += 1,
                '\n' => continue,
                _ => return Err(UnknownNucleotide(nucleotide))
            }
        }
    }

    Ok(profile)
}

/// This function calculates consensus string by given profile
///
/// ## Examples
/// ```
/// use rosalind::cons::*;
///
/// let prof = Profile {
///     A: vec![5, 1, 0, 0, 5, 5, 0, 0],
///     C: vec![0, 0, 1, 4, 2, 0, 6, 1],
///     G: vec![1, 1, 6, 3, 0, 1, 0, 0],
///     T: vec![1, 5, 0, 0, 0, 1, 1, 6],
/// };
///
/// assert_eq!(consensus(prof).unwrap(), "ATGCAACT");
/// ```
pub fn consensus(profile: Profile) -> RosalindResult<String> {
    let buf_capacity = profile.A.len();
    let mut buf: String = String::with_capacity(buf_capacity);

    for i in 0..buf_capacity {
        let mut consensus_char: char = 'A';
        let mut consensus_max: u32 = profile.A[i];

        if profile.C[i] > consensus_max {
            consensus_char = 'C';
            consensus_max = profile.C[i];
        }

        if profile.G[i] > consensus_max {
            consensus_char = 'G';
            consensus_max = profile.G[i];
        }

        if profile.T[i] > consensus_max {
            consensus_char = 'T';
        }

        buf.push(consensus_char);
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::{consensus, profile, Profile};
    use super::super::RosalindError::UnknownNucleotide;

    #[test]
    fn it_should_return_error_when_unknown_nucleotide_found() {
      assert_eq!(profile(vec!["Z"]).unwrap_err(), UnknownNucleotide('Z'));
    }

    #[test]
    fn it_should_calculate_profile() {
        let dna_list = vec![
            "ATCCAGCT",
            "GGGCAACT",
            "ATGGATCT",
            "AAGCAACC",
            "TTGGAACT",
            "ATGCCATT",
            "ATGGCACT",
        ];

        let expected_profile = Profile {
            A: vec![5, 1, 0, 0, 5, 5, 0, 0],
            C: vec![0, 0, 1, 4, 2, 0, 6, 1],
            G: vec![1, 1, 6, 3, 0, 1, 0, 0],
            T: vec![1, 5, 0, 0, 0, 1, 1, 6],
        };

        let prof = profile(dna_list).unwrap();
        assert_eq!(prof, expected_profile);
    }

    #[test]
    fn it_should_calculate_consensus() {
        let prof = Profile {
            A: vec![5, 1, 0, 0, 5, 5, 0, 0],
            C: vec![0, 0, 1, 4, 2, 0, 6, 1],
            G: vec![1, 1, 6, 3, 0, 1, 0, 0],
            T: vec![1, 5, 0, 0, 0, 1, 1, 6],
        };

        let expected_consensus = "ATGCAACT";

        let cens = consensus(prof).unwrap();
        assert_eq!(cens, expected_consensus);
    }
}
