use std::collections::HashMap;
use std::ops;

pub fn is_palindrome(num: u32) -> bool {
    let mut temp = num;
    let mut reversed = 0;

    while temp > 0 {
        reversed *= 10;
        reversed += temp % 10;
        temp /= 10;
    }

    return reversed == num;
}

pub fn get_factorization(mut num: u32) -> HashMap<u32, u32> {
    let mut factors: HashMap<u32, u32> = HashMap::new();

    let mut base: u32 = 2;

    while num > 1 {
        let mut iters = 0;
        while num % base == 0 {
            iters += 1;
            num /= base
        }
        if iters > 0 {
            factors.insert(base, iters);
        }
        base += 1
    }

    factors
}

#[derive(Clone)]
pub struct FactorizedNum {
    factors: HashMap<u32, u32>,
}

impl FactorizedNum {
    pub fn create(x: u32) -> FactorizedNum {
        FactorizedNum {
            factors: get_factorization(x),
        }
    }

    pub fn materialize(&self) -> u64 {
        let mut prod: u64 = 1;
        for (key, val) in &self.factors {
            for _ in 0..*val {
                prod *= *key as u64
            }
        }

        prod
    }
}

impl ops::Mul<&FactorizedNum> for FactorizedNum {
    type Output = FactorizedNum;

    fn mul(self, _rhs: &FactorizedNum) -> FactorizedNum {
        let mut newmap = self.factors.clone();

        for (key, val) in &_rhs.factors {
            newmap.insert(*key, newmap.get(&key).unwrap_or(&0) + val);
        }

        FactorizedNum { factors: newmap }
    }
}

impl ops::Div<&FactorizedNum> for FactorizedNum {
    type Output = Result<FactorizedNum, String>;

    fn div(self, _rhs: &FactorizedNum) -> Self::Output {
        let mut newmap = self.factors.clone();

        for (key, val) in &_rhs.factors {
            let newval = *newmap.get(&key).unwrap_or(&0) as i32 - *val as i32;
            if newval < 0 {
                return Err("not divisible :(".to_string());
            }
            newmap.insert(*key, newval as u32);
        }

        Ok(FactorizedNum { factors: newmap })
    }
}

pub fn sum_divisors(x: u32) -> u32 {
    (1..=(x / 2)).filter(|y| x % y == 0).sum()
}
