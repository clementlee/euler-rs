pub fn solution() -> u32 {
    for a in 1..999 {
        for b in 1..999 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    panic!("should never happen :(")
}
