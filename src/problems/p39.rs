fn get_solutions(p: u32) -> u32 {
    (1..p)
        .flat_map(|x| (1..(p - x)).map(move |y| (x, y)))
        .filter(|(x, y)| {
            let z = p - x - y;
            y * y + z * z == x * x
        })
        .count() as u32
}
pub fn solution() -> u32 {
    (1..=1000)
        .map(|x| (x, get_solutions(x)))
        .max_by_key(|(_i, val)| *val)
        .unwrap()
        .0 as u32
}
