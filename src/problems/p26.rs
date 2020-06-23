fn get_cycle_length(x: u32) -> u32 {
    let mut a = 10 % x;
    let mut t = 0;
    while a != 1 {
        a = a * 10 % x;
        t += 1
    }
    t
}

fn get_primes(x: u32) -> Vec<u32> {
    let mut primes = vec![2];

    for i in (3..x).step_by(2) {
        if !primes.iter().any(|p| i % p == 0) {
            primes.push(i)
        }
    }

    primes
}
pub fn solution() -> u32 {
    let primes = get_primes(1000);

    let x = primes[3..primes.len()]
        .iter()
        .map(|&x| (get_cycle_length(x), x))
        .max_by_key(|&(length, _)| length)
        .unwrap();

    x.1
}
