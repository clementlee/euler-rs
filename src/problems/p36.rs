use crate::lib::get_digits_base;

fn is_palindrome(l: Vec<u32>) -> bool {
    l.iter().eq(l.iter().rev())
}

pub fn solution() -> u32 {
    (1..1_000_000)
        .filter(|&x| {
            let base2 = get_digits_base(x, 2);
            let base10 = get_digits_base(x, 10);

            is_palindrome(base2) && is_palindrome(base10)
        })
        .sum()
}
