use crate::lib::BigInt;
use std::mem;

pub fn solution() -> usize {
    let mut a = BigInt::create(0);
    let mut b = BigInt::create(1);

    let mut i = 1;
    loop {
        if b.digits.len() >= 1_000 {
            break;
        }

        a += &b;
        mem::swap(&mut a, &mut b);
        i += 1;
    }
    i
}
