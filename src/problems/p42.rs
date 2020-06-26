use std::collections::HashSet;
use std::fs;

pub fn solution() -> usize {
    let words: Vec<u32> = fs::read_to_string("data/p042_words.txt")
        .unwrap()
        .split(',')
        .map(|x| x.trim_matches('"'))
        .map(|x| x.chars().map(|ch| ch as u32 - 'A' as u32 + 1).sum())
        .collect();

    let max = *words.iter().max().unwrap();

    let mut cumsum = 0;
    let mut i = 0;
    let mut set: HashSet<u32> = HashSet::new();

    while cumsum <= max {
        i += 1;
        cumsum += i;
        set.insert(cumsum);
    }

    words.iter().filter(|x| set.contains(x)).count()
}
