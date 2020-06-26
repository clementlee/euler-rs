use std::cmp;
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

#[derive(Debug, Clone)]
pub struct BigInt {
    pub digits: Vec<u8>,
}

impl BigInt {
    pub fn create(x: u32) -> BigInt {
        let mut copy = x;
        let mut ret = vec![];

        while copy > 0 {
            ret.push((copy % 10) as u8);
            copy /= 10
        }

        BigInt { digits: ret }
    }
}

impl ops::Add<&BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, other: &BigInt) -> Self {
        let mut new = self.clone();
        new += &other;

        new
    }
}

impl ops::AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, other: &Self) {
        let rhs = &other.digits;

        let digits_sans_carry = cmp::max(self.digits.len(), rhs.len());

        let mut carry = 0;

        for i in 0..digits_sans_carry {
            let sum = self.digits.get(i).unwrap_or(&0) + rhs.get(i).unwrap_or(&0) + carry;

            let base = sum % 10;
            if i < self.digits.len() {
                self.digits[i] = base;
            } else {
                self.digits.push(base);
            }

            carry = (sum - base) / 10;
        }

        if carry > 0 {
            self.digits.push(carry);
        }
    }
}

pub fn get_digits_base(mut x: u32, base: u32) -> Vec<u32> {
    let mut ret = vec![];
    while x > 0 {
        ret.push(x % base);
        x /= base
    }

    ret
}

pub fn get_digits(x: u32) -> Vec<u32> {
    get_digits_base(x, 10)
}

pub fn sieve_primes(x: usize) -> Vec<usize> {
    let mut primes = vec![];
    let mut marker = vec![false; x + 1];
    for i in 2..=x {
        if marker[i] {
            continue;
        }
        primes.push(i);
        marker[i] = true;
        for j in (2 * i..x).step_by(i) {
            marker[j] = true;
        }
    }

    primes
}

pub fn num_digits(x: u64) -> u64 {
    let x = x as f64;
    let x = x.log10() + 1f64;

    x as u64
}

pub fn pandigital(mut x: u64, n: u64, include_zero: bool) -> bool {
    let mut digits = [false; 10];
    while x > 0 {
        let digit = x % 10;
        if (!include_zero && digit == 0) || digit > n {
            return false;
        }
        let digit = digit as usize;
        if digits[digit] {
            return false;
        }

        digits[digit] = true;
        x /= 10;
    }
    digits
        .iter()
        .skip(!include_zero as usize)
        .take(n as usize)
        .all(|&x| x)
}

pub fn permutations(input: Vec<u8>) -> Vec<Vec<u8>> {
    if input.len() <= 1 {
        return vec![input];
    }

    let mut result: Vec<Vec<u8>> = vec![];

    for i in 0..input.len() {
        let first = &input[0..i];
        let second = &input[i + 1..input.len()];

        let val = input[i];

        let mut concat = first.to_vec();
        concat.extend(second.to_vec());

        for perm in permutations(concat) {
            let mut new_perm = vec![val];
            new_perm.extend(perm);

            result.push(new_perm);
        }
    }

    result
}
