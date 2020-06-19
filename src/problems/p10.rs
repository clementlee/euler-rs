const MAX: usize = 2_000_000;
pub fn solution() -> usize {
    let mut prime = [true; MAX];

    let mut sum = 0;

    for i in 2..MAX {
        //(MAX as f64).sqrt() as usize {
        if prime[i] {
            sum += i;
            let mut base = i * i;
            while base < MAX {
                prime[base] = false;
                base += i;
            }
        }
    }

    sum
}
