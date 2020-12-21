use std::collections::{HashMap, HashSet};

use crate::lib::input::load_input;
use itertools::Itertools;

type Food = (HashSet<String>, HashSet<String>);

fn parse_input(input: &str) -> Vec<Food> {
    let mut data = vec![];
    for line in input.lines() {
        let (a, b) = line.split(" (").next_tuple().unwrap();
        let ingredients = a
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<HashSet<String>>();
        let allergens = b
            .replace("contains ", "")
            .replace(")", "")
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<HashSet<String>>();
        data.push((ingredients, allergens));
    }
    return data;
}

fn part_one(data: &Vec<Food>) -> HashMap<String, String> {
    // println!("{:#?}", data);
    let mut possibilities: HashMap<String, Vec<String>> = HashMap::new();
    // allergen -> ingredient
    let mut resolved: HashMap<String, String> = HashMap::new();

    for food in data
        .iter()
        .sorted_by(|&a, &b| Ord::cmp(&a.0.len(), &b.0.len()))
    {
        let (ingr, allerg) = food;
        for a in allerg {
            if resolved.contains_key(a) {
                continue;
            }

            let mut shared_ingredients = ingr.to_owned();
            let similar_allergs = data
                .iter()
                .filter(|&f| f != food)
                .filter(|(_, fa)| fa.contains(a))
                .map(|(fi, _)| fi)
                .collect::<Vec<&HashSet<String>>>();

            // println!("similar allergs: {:#?}", similar_allergs);

            for &sa in similar_allergs.iter() {
                shared_ingredients = shared_ingredients
                    .intersection(sa)
                    .map(|s| s.to_owned())
                    .collect();
            }

            if shared_ingredients.len() == 1 {
                resolved.insert(
                    a.to_owned(),
                    shared_ingredients.iter().next().unwrap().to_owned(),
                );
            } else {
                let mut v = vec![];
                for i in &shared_ingredients {
                    v.push(i.to_owned());
                }
                possibilities.insert(a.to_owned(), v);
            }

            for r in resolved.iter() {
                let (r_a, r_i) = r;
                for (p_a, p_ingr) in possibilities.iter_mut().filter(|(a, i)| i.contains(r_i)) {
                    *p_ingr = p_ingr
                        .iter()
                        .filter(|&i| i != r_i)
                        .map(|s| s.to_owned())
                        .collect();
                }
            }
        }
    }

    while possibilities.len() > 0 {
        let mut found_ing = vec![];
        for (a, i) in possibilities.iter().filter(|(a, i)| i.len() == 1) {
            resolved.insert(a.to_owned(), i[0].to_owned());
            found_ing.push(i[0].to_owned());
        }
        possibilities = possibilities
            .iter()
            .filter(|(a, i)| i.len() > 1)
            .map(|(a, i)| {
                (
                    a.to_owned(),
                    i.iter()
                        .filter(|&x| !found_ing.contains(x))
                        .map(|x| x.to_owned())
                        .collect(),
                )
            })
            .collect();
    }

    // println!("resolved: {:#?}", resolved);
    // println!("posibilities: {:#?}", possibilities);

    let res_vals = resolved.values().collect::<Vec<&String>>();

    let ingr_wo_allergens = data
        .iter()
        .map(|(ingr, allerg)| {
            ingr.iter()
                .filter(|i| !res_vals.contains(i))
                .collect::<Vec<&String>>()
        })
        .flatten()
        .collect::<Vec<&String>>();

    println!("Part 1: {}", ingr_wo_allergens.len());
    resolved
}

fn part_two(resolved: &HashMap<String, String>) {
    let result_str = resolved
        .iter()
        .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        .map(|(a, i)| i)
        .join(",");
    println!("Part 2: {}", result_str);
}

pub fn run() {
    let input = load_input("src/solutions/21/data.txt");
    let data: Vec<Food> = parse_input(&input);
    let resolved = part_one(&data);
    part_two(&resolved);
}
