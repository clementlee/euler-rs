use crate::lib::pandigital;
const MAX: u64 = 1_000_000_000;

fn create_num(x: u64, y: u64) -> Option<u64> {
    (1..=y).fold(Some(0), |acc, n| match acc {
        Some(acc) => {
            let prod = n * x;
            let digits = (prod as f64).log10() as u32 + 1;

            let prod = acc * 10u64.pow(digits) + prod;
            if prod > MAX {
                return None;
            } else {
                return Some(prod);
            }
        }
        None => None,
    })
}

pub fn solution() -> u64 {
    (1..100_000)
        .flat_map(|x| (1..10).map(move |y| (x, y)))
        .filter_map(|(x, y)| create_num(x, y))
        .filter(|&x| pandigital(x, 9, false))
        .max()
        .unwrap()
}
