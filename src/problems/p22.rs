use std::fs;

fn get_value(x: &str) -> u64 {
    x.chars().map(|c| c as u64 - 'A' as u64 + 1).sum()
}

pub fn solution() -> u64 {
    let names = fs::read_to_string("data/p022_names.txt").unwrap();
    let mut names: Vec<&str> = names.split(',').map(|x| x.trim_matches('"')).collect();
    names.sort();

    let sum: u64 = names
        .iter()
        .enumerate()
        .map(|(i, x)| ((i + 1) as u64 * get_value(x)) as u64)
        .sum();
    sum
}
