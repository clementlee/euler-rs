use crate::lib::get_digits;

pub fn solution() -> u32 {
    let factorials: Vec<u32> = (0..=9).map(|x| (1..=x).product()).collect();

    let max = factorials[factorials.len() - 1];
    let max = get_digits(max).len() as u32 * max;

    (10..max)
        .filter(|&x| {
            get_digits(x)
                .iter()
                .map(|&y| factorials[y as usize])
                .sum::<u32>()
                == x
        })
        .sum()
}
