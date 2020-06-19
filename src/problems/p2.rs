pub fn solution() -> u32 {
    let limit: u32 = 4_000_000;

    let mut i1: u32 = 1;
    let mut i2: u32 = 1;

    let mut sum: u32 = 0;

    loop {
        let temp = i2;
        i2 += i1;
        i1 = temp;

        if i2 > limit {
            break;
        } else if i2 % 2 == 0 {
            sum += i2
        }
    }

    sum
}
