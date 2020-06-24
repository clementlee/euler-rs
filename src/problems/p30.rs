const POW: u32 = 5;

use crate::lib::get_digits;

pub fn solution() -> u32 {
    let max = 9u32.pow(POW);
    let max = get_digits(max).len() as u32 * max;

    (2u32..=max)
        .filter(|x| get_digits(*x).iter().map(|&y| y.pow(POW)).sum::<u32>() == *x)
        .sum()
}
