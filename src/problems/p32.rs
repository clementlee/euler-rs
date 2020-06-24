use std::collections::HashSet;

fn pandigital(x: u32, y: u32) -> bool {
    let mult = x * y;

    let mut digits: [bool; 9] = [false; 9];

    for copy in [x, y, mult].iter_mut() {
        let mut num = *copy;
        while num > 0 {
            let modulus = (num % 10) as usize;
            if modulus == 0 || digits[modulus - 1] {
                return false;
            } else {
                digits[modulus - 1] = true;
            }
            num /= 10
        }
    }

    digits.iter().all(|&x| x)
}
pub fn solution() -> u32 {
    let limit = 10_000;
    (1..limit)
        .flat_map(|x| (1..limit).map(move |y| (x, y)))
        .filter(|&(x, y)| pandigital(x, y))
        .map(|(x, y)| x * y)
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
