use crate::lib;

fn triangle(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn get_num_divisors(n: u32) -> u32 {
    lib::get_factorization(n as u32)
        .iter()
        .fold(1, |acc, (_, y)| acc * (y + 1))
}

pub fn solution() -> u32 {
    let mut n = 1;
    loop {
        let tri_val = triangle(n);
        let divisors = get_num_divisors(tri_val);
        if divisors > 500 {
            return tri_val;
        }

        n += 1
    }
}
