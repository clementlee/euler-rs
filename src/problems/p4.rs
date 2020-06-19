use crate::lib;
use std::cmp;

pub fn solution() -> u32 {
    let mut best = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let prod = i * j;
            if lib::is_palindrome(prod) {
                best = cmp::max(best, prod)
            }
        }
    }
    best
}
