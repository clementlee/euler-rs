use std::collections::HashSet;
pub fn solution() -> usize {
    (2..=100)
        .flat_map(|a| (2..=100).map(move |b| (a, b)))
        .map(|(a, b)| {
            let a = a as f64;
            let b = b as f64;
            a.powf(b)
        })
        .map(|x| format!("{}", x)) // dirty hack to allow hashing...
        .collect::<HashSet<String>>()
        .len()
}
