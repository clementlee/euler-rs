struct BigNum {
    nums: Vec<u8>,
}

impl BigNum {
    fn new() -> BigNum {
        BigNum { nums: vec![1] }
    }

    fn sum_digits(&self) -> u32 {
        self.nums.iter().fold(0u32, |acc, x| acc + (*x as u32))
    }

    fn mul_two(&mut self) {
        let mut carry = 0;
        for i in 0..self.nums.len() {
            let temp = self.nums[i] * 2 + carry;
            let base = temp % 10;
            carry = (temp - base) / 10;
            self.nums[i] = base
        }
        if carry > 0 {
            self.nums.push(carry)
        }
    }
}

const ITERS: u32 = 1000;

pub fn solution() -> u32 {
    let mut tmp = BigNum::new();

    for _ in 0..ITERS {
        tmp.mul_two();
    }

    tmp.sum_digits()
}
