use std::collections::HashMap;

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
