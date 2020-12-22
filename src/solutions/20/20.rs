use std::collections::HashSet;

use crate::lib::input::load_input;

#[derive(Debug, Clone, PartialEq)]
struct Tile {
    id: u32,
    orientation: u32,
    sides: Vec<u32>,
    body: Vec<Vec<char>>,
}

fn fold_side(side: &Vec<char>) -> u32 {
    let bin_str = side.iter().fold(String::new(), |acc, x| {
        if *x == '#' {
            format!("{}{}", acc, "1")
        } else {
            format!("{}{}", acc, "0")
        }
    });
    u32::from_str_radix(&bin_str, 2).unwrap()
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let id = lines
                .next()
                .unwrap()
                .replace("Tile ", "")
                .replace(":", "")
                .parse::<u32>()
                .unwrap();
            let body = lines
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            let top = fold_side(&body[0]);
            let right = fold_side(&body.iter().map(|l| l[l.len() - 1]).collect());
            let bot = fold_side(&body[body.len() - 1]);
            let left = fold_side(&body.iter().map(|l| l[0]).collect());

            Tile {
                id,
                body,
                sides: vec![top, right, bot, left],
                orientation: 0,
            }
        })
        .collect()
}

fn part_one(tiles: &Vec<Tile>) {
    let corners = tiles
        .iter()
        .filter(|&tile| {
            let set = tile.sides.iter().map(|s| *s).collect::<HashSet<u32>>();
            tiles
                .iter()
                .filter(|&t| t.id != tile.id)
                .filter(|&t| {
                    let t_set = t.sides.iter().map(|s| *s).collect::<HashSet<u32>>();
                    set.intersection(&t_set).count() > 0
                })
                .count()
                == 2
        })
        .count();
    println!("{}", corners);
}

pub fn run() {
    let input = load_input("src/solutions/20/data.txt");
    let data = parse_input(&input);
    println!(
        "{:#?}",
        data.iter()
            .map(|t| (t.id, t.sides.clone()))
            .collect::<Vec<(u32, Vec<u32>)>>()
    );
    part_one(&data);
}
