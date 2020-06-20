use std::collections::HashMap;

fn collatz(x: u64) -> u64 {
    match x % 2 {
        0 => x / 2,
        1 => 3 * x + 1,
        _ => panic!("impossible"),
    }
}

const MAX: u64 = 1_000_000;

pub fn solution() -> u64 {
    let mut longest_i = 0;
    let mut longest = 0;
    let mut cache: HashMap<u64, u64> = HashMap::new();

    for i in 1..MAX {
        let mut copy = i;
        let mut chain = vec![];

        let mut base = 0;
        loop {
            if let Some(chain_length) = cache.get(&copy) {
                base = *chain_length;
                break;
            }
            chain.push(copy);
            if copy == 1 {
                break;
            }
            copy = collatz(copy);
        }

        for j in 0..chain.len() {
            cache.insert(chain[j], base + chain.len() as u64 - j as u64);
        }

        let current_length = chain.len() as u64 + base;
        if current_length > longest {
            longest = current_length;
            longest_i = i;
        }
    }

    longest_i
}
