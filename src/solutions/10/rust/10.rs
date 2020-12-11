use std::collections::HashMap;

use crate::lib::input::load_input;

fn calculate_differences(input: &Vec<u32>) {
    let mut oneJolt = 1;
    let mut threeJolt = 1;

    for i in 1..input.len() {
        if input[i] - input[i - 1] == 1 {
            oneJolt += 1;
        } else if input[i] - input[i - 1] == 3 {
            threeJolt += 1;
        } else if input[i] - input[i - 1] == 2 {
        }
    }
    println!("Part 1: {}", oneJolt * threeJolt);
}

fn find_variations(full_input: &Vec<u32>, i: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if i == full_input.len() - 1 {
        return 1;
    }
    if let Some(value) = cache.get(&i) {
        return *value;
    }
    let mut cases: usize = 0;
    for j in i + 1..full_input.len() {
        if full_input[j] - full_input[i] <= 3 {
            cases += find_variations(&full_input, j, cache);
        }
    }
    cache.insert(i, cases);
    return cases;
}

pub fn run() {
    let mut input: Vec<u32> = load_input("src/solutions/10/data.txt")
        .lines()
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect();
    input.sort();
    calculate_differences(&input);

    let mut full_input = input.clone();
    full_input.insert(0, 0);
    full_input.insert(full_input.len(), full_input[full_input.len() - 1] + 3);
    let mut cache = HashMap::new();
    let cases = find_variations(&full_input, 0, &mut cache);
    println!("Part 2: {}", cases);
}
