pub fn solution() -> u32 {
    let mut primes = vec![2u32, 3, 5, 7];

    let mut special_primes: Vec<u32> = vec![];
    let mut i = 10;

    while special_primes.len() < 11 {
        i += 1;
        let isprime = !primes.iter().any(|&x| i % x == 0);
        if isprime {
            primes.push(i);

            let mut copy = i;
            let mut is_left_prime = true;
            while copy > 0 {
                if primes.binary_search(&copy).is_err() {
                    is_left_prime = false;
                    break;
                }
                copy /= 10;
            }

            copy = i;
            let shift = (copy as f64).log10() as u32 + 1;
            let mut is_right_prime = true;
            for j in (1..=shift).rev() {
                let modulus = 10u32.pow(j);
                copy %= modulus;
                if primes.binary_search(&copy).is_err() {
                    is_right_prime = false;
                    break;
                }
            }

            if is_left_prime && is_right_prime {
                special_primes.push(i)
            }
        }
    }

    special_primes.iter().sum()
}
