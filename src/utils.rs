//! Module with useful utilities

use RosalindResult;
use constants::FASTA_LABEL_SYMBOL;

fn is_fasta_label(s: &str) -> bool {
    s.contains(FASTA_LABEL_SYMBOL)
}

/// This parses dataset in FASTA format into array of DNA strings
///
/// ## Examples
/// ```
/// use rosalind::utils::*;
///
/// let fasta_dataset = ">Rosalind_1
///     CCTGCGGAAG
///     TCCCACTAAT
///     >Rosalind_2
///     CCATCGGTAG
///     ATATCCATTT
///     >Rosalind_3
///     CCACCCTCGT
///     TGGGAACCTG";
///
/// let expected_dataset = vec![
///     "CCTGCGGAAGTCCCACTAAT",
///     "CCATCGGTAGATATCCATTT",
///     "CCACCCTCGTTGGGAACCTG",
/// ];
///
/// assert_eq!(parse_fasta_dataset(fasta_dataset).unwrap(), expected_dataset);
/// ```
pub fn parse_fasta_dataset(dataset: &str) -> RosalindResult<Vec<String>> {
    let delimiter: char = '\x00';
    let mut buf: String = String::new();

    for dataset_line in dataset.lines() {
        if is_fasta_label(dataset_line) {
            if !buf.is_empty() {
                buf.push(delimiter);
            }
        } else {
            buf.push_str(dataset_line.trim());
        }
    }

    Ok(buf.split(delimiter).map(|s| s.to_string()).collect())
}

#[test]
fn it_should_determine_fasta_label() {
    assert_eq!(is_fasta_label(">Rosalind_1"), true);
    assert_eq!(is_fasta_label("CCTGCGGAAG"), false);
}

#[cfg(test)]
mod tests {
    use super::parse_fasta_dataset;

    #[test]
    fn it_should_parse_fasta_dataset() {
        let fasta_dataset = ">Rosalind_1
            CCTGCGGAAG
            TCCCACTAAT
            >Rosalind_2
            CCATCGGTAG
            ATATCCATTT
            >Rosalind_3
            CCACCCTCGT
            TGGGAACCTG";

        let expected_dataset = vec![
            "CCTGCGGAAGTCCCACTAAT",
            "CCATCGGTAGATATCCATTT",
            "CCACCCTCGTTGGGAACCTG",
        ];

        let dataset = parse_fasta_dataset(fasta_dataset).unwrap();

        assert_eq!(dataset, expected_dataset);
    }
}
