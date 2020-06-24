const POW: u32 = 5;

fn get_digits(mut x: u32) -> Vec<u32> {
    let mut ret = vec![];
    while x > 0 {
        ret.push(x % 10);
        x /= 10
    }

    ret
}

pub fn solution() -> u32 {
    let max = 9u32.pow(POW);
    let max = get_digits(max).len() as u32 * max;

    (2u32..=max)
        .filter(|x| get_digits(*x).iter().map(|&y| y.pow(POW)).sum::<u32>() == *x)
        .sum()
}
