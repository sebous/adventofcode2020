use crate::lib::input::load_input;
use std::collections::HashMap;

type Bagmap = HashMap<String, HashMap<String, u32>>;
const SHINY_GOLD: &str = "shiny gold";

fn parse_input(input: String) -> Bagmap {
    let mut map: Bagmap = HashMap::new();
    for line in input.lines() {
        let mut halfs = line.split("contain");
        let parent = halfs.next().unwrap().replace("bags", "");
        let children = halfs.next().unwrap().replace(".", "");
        let mut children_map: HashMap<String, u32> = HashMap::new();
        for child in children.split(",") {
            if child.contains("no other bags") {
                continue;
            }
            let parts: Vec<&str> = child.trim().split(" ").collect();
            let key = format!("{} {}", parts[1], parts[2]);
            let value: u32 = parts[0].parse().unwrap();
            children_map.insert(key, value);
        }
        map.insert(String::from(parent.trim()), children_map);
    }
    return map;
}

fn does_color_contain_gold(map: &Bagmap, color: &String) -> bool {
    let relationship = map.get(color).unwrap();
    if relationship.contains_key(SHINY_GOLD) {
        return true;
    }
    for c in relationship.keys() {
        let has_gold = does_color_contain_gold(map, c);
        if has_gold {
            return true;
        }
    }
    return false;
}

fn part1(map: &Bagmap) {
    let mut counter: u32 = 0;
    for color in map.keys() {
        if does_color_contain_gold(map, color) {
            counter += 1;
        }
    }
    println!("Part 1: {}", counter);
}

fn count_bags_included(map: &Bagmap, color: &String) -> u32 {
    let mut bags_count: u32 = 0;
    let relationship = map.get(color).unwrap();
    for (c, count) in relationship.into_iter() {
        bags_count += count + count * count_bags_included(map, c);
    }
    return bags_count;
}

pub fn run() {
    let input = load_input("src/solutions/07/data.txt");
    let map: Bagmap = parse_input(input);
    part1(&map);
    let bags_part_2 = count_bags_included(&map, &String::from(SHINY_GOLD));
    println!("Part 2: {}", bags_part_2);
}
