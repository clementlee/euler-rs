pub fn solution() -> usize {
    let n = 10_001;
    let mut primes = vec![2];

    let mut i = 3;
    while primes.len() < n {
        let mut is_prime = true;
        for prime in primes.iter() {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i += 2
    }

    primes[n - 1]
}
