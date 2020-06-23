fn is_prime(x: u32, mut cached_primes: Vec<u32>) -> (bool, Vec<u32>) {
    let cap = x as f64;
    let cap = cap.sqrt();
    let cap = cap as u32;

    let highest_prime = cached_primes[cached_primes.len() - 1];
    if highest_prime <= cap {
        let mut val = highest_prime;
        loop {
            val += 1;
            if !cached_primes.iter().any(|p| val % p == 0) {
                cached_primes.push(val);
                if val > cap {
                    break;
                }
            }
        }
    }

    let yay = !cached_primes.iter().any(|p| x % p == 0);
    (yay, cached_primes)
}

pub fn solution() -> i32 {
    let mut primes = vec![2];

    let mut max_primes = 0;
    let mut max_product = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            let mut n = 0;
            loop {
                let val = n * n + a * n + b;
                if val < 0 {
                    break;
                }
                let val = val as u32;
                let ret = is_prime(val, primes);
                primes = ret.1;
                if !ret.0 {
                    break;
                } else {
                    n += 1
                }
            }

            if n > max_primes {
                max_primes = n;
                max_product = a * b;
            }
        }
    }

    println!("{:?}", primes);

    max_product
}
