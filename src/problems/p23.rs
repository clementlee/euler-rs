use crate::lib::sum_divisors;
const UPPER_BOUND: u32 = 28123;

pub fn solution() -> u32 {
    let abundant_nums: Vec<u32> = (1..UPPER_BOUND).filter(|x| sum_divisors(*x) > *x).collect();

    let mut abundant_sums = [false; UPPER_BOUND as usize];
    for i in &abundant_nums {
        for j in &abundant_nums {
            let sum = i + j;
            if sum < UPPER_BOUND {
                abundant_sums[sum as usize] = true;
            }
        }
    }

    abundant_sums
        .iter()
        .enumerate()
        .filter(|(_i, x)| !**x)
        .map(|(i, _x)| i as u32)
        .sum()
}
