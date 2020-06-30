use crate::lib::num_digits;
use crate::lib::pandigital;
use crate::lib::sieve_primes;
pub fn solution() -> usize {
    sieve_primes(1_000_000_000)
        .iter()
        .rev()
        .cloned()
        .find(|&x| pandigital(x as u64, num_digits(x as u64), false))
        .unwrap()
}
