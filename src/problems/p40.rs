use crate::lib::get_digits;
struct Cham {
    counter: i32,
    sub: u32,
}

impl Iterator for Cham {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        let mut output = vec![];
        if self.counter > 0 {
            output.extend(get_digits(self.counter as u32).iter().rev());
        }
        output.push(self.sub);

        self.sub += 1;
        if self.sub > 9 {
            self.counter += 1;
            self.sub = 0;
        }

        Some(output)
    }
}

pub fn solution() -> u32 {
    let cham = Cham { counter: 0, sub: 0 };
    let mut vals = cham.into_iter().flat_map(|x| x.into_iter()).skip(1);

    let needs = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];
    let mut prev = 0;

    let mut prod = 1;
    for &need in needs.iter() {
        let newval = vals.nth(need - prev - 1).unwrap();
        prod *= newval;
        prev = need
    }

    prod
}
