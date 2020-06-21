use std::collections::HashMap;

fn sum_divisors(x: u32) -> u32 {
    (1..=(x / 2)).filter(|y| x % y == 0).sum()
}

pub fn solution() -> u32 {
    let cache: HashMap<u32, u32> = (1..10000).map(|x| (x, sum_divisors(x))).collect();

    let mut sum = 0;
    for (key, value) in &cache {
        let amicable: bool = match cache.get(&value) {
            Some(val) => *key == *val && key != value,
            None => false,
        };

        if amicable {
            sum += key;
        }
    }
    sum
}
