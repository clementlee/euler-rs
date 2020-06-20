fn stringify(x: u32) -> String {
    let ret: String = match x {
        0 => "".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        21..=99 => {
            let first = stringify(x / 10 * 10);
            let second = stringify(x % 10);

            [first, second].concat()
        }
        100..=999 => {
            let first = stringify(x / 100);
            let mut next = stringify(x % 100);
            if next.len() != 0 {
                next = ["and".to_string(), next].concat();
            }

            [first, "hundred".to_string(), next].concat()
        }
        1000..=9999 => {
            let first = stringify(x / 1000);
            let next = stringify(x % 1000);

            [first, "thousand".to_string(), next].concat()
        }
        _n => panic!("bad"),
    };

    ret
}
pub fn solution() -> usize {
    (1..=1000).map(|x| stringify(x).len()).sum()
}
