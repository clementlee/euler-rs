use std::cmp;
fn gcd(x: u32, y: u32) -> u32 {
    let min = cmp::min(x, y);
    let max = cmp::max(x, y);

    let modulus = max % min;
    if modulus == 0 {
        return min;
    } else {
        return gcd(min, modulus);
    }
}
pub fn solution() -> u32 {
    let mut x = 1;
    let mut y = 1;

    for i in 1..10 {
        for den in 1..i {
            for num in 1..den {
                if (10 * num + i) * den == (10 * i + den) * num {
                    x *= num;
                    y *= den;
                }
            }
        }
    }

    y / gcd(x, y)
}
