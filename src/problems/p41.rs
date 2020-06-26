use crate::lib::num_digits;
use crate::lib::pandigital;
use crate::lib::sieve_primes;
pub fn solution() -> usize {
    sieve_primes(1_000_000_000)
        .iter()
        .rev()
        .map(|&x| x)
        .filter(|&x| pandigital(x as u64, num_digits(x as u64), false))
        .nth(0)
        .unwrap()
}
