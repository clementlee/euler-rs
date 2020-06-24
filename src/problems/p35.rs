use std::collections::HashSet;

const MAX: usize = 1_000_000;
pub fn solution() -> u32 {
    let mut primes = vec![true; MAX];
    for i in 2..MAX {
        let mut base = 2 * i;
        while base < MAX {
            primes[base] = false;
            base += i
        }
    }

    let primes: Vec<u32> = primes
        .iter()
        .enumerate()
        .filter_map(|(x, y)| match y {
            true => Some(x as u32),
            false => None,
        })
        .collect();

    let primes_set: HashSet<u32> = primes[2..primes.len()].iter().map(|&x| x).collect();

    let mut count = 0;
    for &prime in primes_set.iter() {
        let mut val = prime;
        let mut allprime = true;
        let shift = val as f64;
        let shift = shift.log10() as u32;
        for _ in 0..shift {
            let digit = val % 10;
            val /= 10;
            val += 10u32.pow(shift) * digit;

            if !&primes_set.contains(&val) {
                allprime = false;
                break;
            }
        }

        if allprime {
            count += 1
        }
    }

    count
}
