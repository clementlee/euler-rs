use crate::lib;
use std::cmp;
use std::collections::HashMap;
pub fn solution() -> u32 {
    let mut factors: HashMap<u32, u32> = HashMap::new();

    for i in 1..21 {
        let factorization = lib::get_factorization(i);

        // merge factorization into factors
        // we only want to take the max of the current times value
        for (factor, times) in factorization.iter() {
            factors.insert(*factor, *cmp::max(factors.get(factor).unwrap_or(&0), times));
        }
    }

    let mut prod = 1;
    for (factor, times) in factors.iter() {
        for _ in 0..*times {
            prod *= factor;
        }
    }

    prod
}
