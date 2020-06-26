use crate::lib::permutations;
use crate::lib::sieve_primes;
pub fn solution() -> u64 {
    let things = sieve_primes(17);
    permutations((0..=9).collect())
        .iter()
        .filter(|&x| x[0] != 0)
        .map(|x| x.iter().fold(0, |acc, &x| acc * 10 + (x as u64)))
        .filter(|&copy| {
            let mut x = copy;
            for &thing in things.iter().rev() {
                if (x % 1000) % thing as u64 != 0 {
                    return false;
                }
                x /= 10;
            }
            return true;
        })
        .sum()
}
