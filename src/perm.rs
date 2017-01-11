//! Module for `Enumerating gene orders`

use utils::factorial;

fn get_permutations(n: u8) -> Vec<u8> {
    unimplemented!()
}

pub fn permutations(n: u8) -> Vec<u8> {
    match n {
        0 | 1 => vec![n],
        _ => get_permutations(n)
    }
}

#[cfg(test)]
mod tests {
    use super::permutations;

    #[test]
    fn it_should_return_permutations_for_01() {
        assert_eq!(permutations(0), vec![0]);
        assert_eq!(permutations(1), vec![1]);
    }

    // #[test]
    // fn it_should_return_permutations_for_() {
    //     assert_eq!(permutations(0), vec![0]);
    //     assert_eq!(permutations(1), vec![1]);
    // }
}
