pub fn solution() -> u32 {
    let mut sum: u32 = 0;

    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n
        }
    }

    sum
}
