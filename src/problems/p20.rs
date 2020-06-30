use std::ops;

#[derive(Debug)]
struct BigNum {
    nums: Vec<u32>,
}

impl ops::MulAssign<u32> for BigNum {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn mul_assign(&mut self, _rhs: u32) {
        let mut carry = 0;
        for i in 0..self.nums.len() {
            let digitsum = self.nums[i] * _rhs + carry;

            let base = digitsum % 10;

            self.nums[i] = base;

            carry = (digitsum - base) / 10;
        }
        while carry > 0 {
            self.nums.push(carry % 10);
            carry /= 10;
        }
    }
}

pub fn solution() -> u32 {
    let mut base = BigNum { nums: vec![1] };

    for i in 2..=100 {
        base *= i
    }

    base.nums.iter().sum()
}
