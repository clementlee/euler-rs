pub fn solution() -> usize {
    let n = 100;

    let mut first = 0;
    for i in 1..(n + 1) {
        first += i * i
    }

    let mut second = n * (n + 1) / 2;
    second *= second;

    second - first
}
