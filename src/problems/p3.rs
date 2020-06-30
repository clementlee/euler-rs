pub fn solution() -> u64 {
    let mut num: u64 = 600851475143;

    let mut largest_factor: u64 = 0;
    let mut base: u64 = 2;

    while num > 1 {
        while num % base == 0 {
            largest_factor = std::cmp::max(largest_factor, base);
            num /= base
        }
        base += 1
    }

    largest_factor
}
