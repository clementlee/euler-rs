use crate::lib::permutations;

/// this is a pretty gross solution...
pub fn solution() -> u64 {
    let elt = &permutations((0..=9).collect::<Vec<u8>>())[999_999];

    elt.iter().fold(0, |acc, x| acc * 10 + *x as u64)
}
