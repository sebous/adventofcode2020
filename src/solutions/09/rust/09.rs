use crate::lib::input::load_input;
use itertools::Itertools;

fn unique_combinations(slice: &[u64]) -> Vec<Vec<u64>> {
    let combinations: Vec<Vec<u64>> = slice
        .iter()
        .map(|n| n.to_owned())
        .combinations(2)
        .unique()
        .collect();
    return combinations;
}

fn find_first_invalid(data: &Vec<u64>, preamble: usize) -> u64 {
    for (i, num) in data.iter().enumerate() {
        if i <= preamble {
            continue;
        }
        let combinations = unique_combinations(&data[i - preamble..i]);
        let is_invalid = combinations.iter().all(|comb| {
            if (comb[0] + comb[1]) != num.to_owned() {
                return true;
            }
            return false;
        });
        if is_invalid {
            return num.to_owned();
        }
    }
    return 0;
}

fn find_range_that_sums_to(data: &Vec<u64>, sum: u64) -> &[u64] {
    for i in 0..data.len() - 1 {
        for j in i + 1..data.len() - 1 {
            let range = &data[i..j];
            if range.iter().sum::<u64>() == sum {
                return range;
            }
        }
    }
    panic!("not found");
}

pub fn run() {
    let input: Vec<u64> = load_input("src/solutions/09/data.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let part1 = find_first_invalid(&input, 25);
    println!("Part 1: {}", part1);
    let mut part2 = find_range_that_sums_to(&input, part1).to_owned();
    part2.sort();
    println!("Part 2: {}", part2[0] + part2[part2.len() - 1]);
}
