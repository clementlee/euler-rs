use std::collections::HashMap;
use std::collections::HashSet;

fn get_str(map: &HashMap<u32, u32>) -> String {
    let mut keys: Vec<&u32> = map.keys().collect();
    keys.sort();
    keys.iter()
        .map(|x| format!("{}x{}", x, map.get(x).unwrap()))
        .collect::<Vec<String>>()
        .join(",")
}

const OPTIONS: [u32; 7] = [2, 5, 10, 20, 50, 100, 200];
pub fn solution() -> usize {
    let mut queue = vec![];

    let mut combinations: HashSet<String> = HashSet::new();

    let mut initial = HashMap::new();
    initial.insert(1, 200);

    queue.push(initial);

    while let Some(top) = queue.pop() {
        let strtop = get_str(&top);
        combinations.insert(strtop);

        if let Some(&ones) = top.get(&1) {
            for &option in OPTIONS.iter() {
                if ones >= option {
                    let mut new = top.clone();
                    new.insert(1, ones - option);
                    new.insert(option, new.get(&option).unwrap_or(&0) + 1);

                    if !combinations.contains(&get_str(&new)) {
                        queue.push(new);
                    }
                }
            }
        }
    }

    combinations.len()
}
