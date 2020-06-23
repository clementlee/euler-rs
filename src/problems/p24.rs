fn permutations(input: Vec<u32>) -> Vec<Vec<u32>> {
    if input.len() <= 1 {
        return vec![input];
    }

    let mut result: Vec<Vec<u32>> = vec![];

    for i in 0..input.len() {
        let first = &input[0..i];
        let second = &input[i + 1..input.len()];

        let val = input[i];

        let mut concat = first.to_vec();
        concat.extend(second.to_vec());

        for perm in permutations(concat) {
            let mut new_perm = vec![val];
            new_perm.extend(perm);

            result.push(new_perm);
        }
    }

    result
}

/// this is a pretty gross solution...
pub fn solution() -> u64 {
    let elt = &permutations((0..=9).collect::<Vec<u32>>())[999_999];

    elt.iter().fold(0, |acc, x| acc * 10 + *x as u64)
}
