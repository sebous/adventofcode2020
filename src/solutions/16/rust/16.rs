use std::collections::HashMap;

use crate::lib::input::load_input;
use itertools::Itertools;

#[derive(Debug)]
struct Rule {
    name: String,
    x: (u32, u32),
    y: (u32, u32),
}

fn parse_input(input: &String) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let mut lines = input.lines();

    let mut rule_lines = vec![];
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        rule_lines.push(line);
    }
    let rules: Vec<Rule> = rule_lines
        .iter()
        .map(|&l| {
            let (name, vals) = l.split(": ").next_tuple().unwrap();
            let (r1, r2) = vals.split(" or ").next_tuple().unwrap();
            let (x1, x2): (u32, u32) = r1
                .split("-")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (y1, y2): (u32, u32) = r2
                .split("-")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            return Rule {
                name: name.to_owned(),
                x: (x1, x2),
                y: (y1, y2),
            };
        })
        .collect();

    lines.next();
    let my_ticket: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let rest = lines.skip(2);
    let mut tickets = vec![];
    for line in rest {
        let ticket: Vec<u32> = line.split(",").map(|n| n.parse().unwrap()).collect();
        tickets.push(ticket);
    }

    return (rules, my_ticket, tickets);
}

fn part_one(data: &(Vec<Rule>, Vec<u32>, Vec<Vec<u32>>)) -> Vec<u32> {
    let (rules, _, tickets) = data;
    let mut invalid = vec![];

    for &n in tickets.iter().flatten() {
        let valid = rules.iter().any(|rule| {
            let (x1, x2) = rule.x;
            let (y1, y2) = rule.y;
            if (n >= x1 && n <= x2) || (n >= y1 && n <= y2) {
                return true;
            }
            return false;
        });
        if !valid {
            invalid.push(n);
        }
    }

    println!("Part 1: {}", invalid.iter().sum::<u32>());
    return invalid;
}

fn part_two(data: &(Vec<Rule>, Vec<u32>, Vec<Vec<u32>>), invalid: &Vec<u32>) {
    let (rules, my_ticket, tickets) = data;
    let tickets_validated: Vec<Vec<u32>> = tickets
        .iter()
        .filter(|ticket| !ticket.iter().any(|n| invalid.contains(n)))
        .map(|t| t.to_owned())
        .collect();

    let mut possibilities: HashMap<String, Vec<usize>> = HashMap::new();

    for i in 0..my_ticket.len() {
        let column: Vec<u32> = tickets_validated.iter().map(|ticket| ticket[i]).collect();
        for rule in rules.iter() {
            let (x1, x2) = rule.x;
            let (y1, y2) = rule.y;
            let rule_match = column.iter().all(|&n| {
                if (n >= x1 && n <= x2) || (n >= y1 && n <= y2) {
                    return true;
                }
                return false;
            });
            if rule_match {
                if let Some(pair) = possibilities.get_mut(&rule.name) {
                    pair.push(i);
                } else {
                    possibilities.insert(rule.name.clone(), vec![i]);
                }
            }
        }
    }

    let available_fields: Vec<String> = possibilities
        .iter()
        .map(|(k, v)| (k, v.len()))
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .into_iter()
        .map(|(k, _)| k.clone())
        .collect();

    let mut matched: HashMap<String, usize> = HashMap::new();
    for field in &available_fields {
        let col_matched = possibilities.get(field).unwrap()[0];
        matched.insert(field.to_owned(), col_matched);
        possibilities.remove(field);
        for val in possibilities.values_mut() {
            *val = val
                .iter()
                .filter(|&col| *col != col_matched)
                .map(|col| col.clone())
                .collect::<Vec<usize>>();
        }
        // println!("{:?}", matched);
    }

    let result: u128 = matched
        .into_iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| v)
        .fold(1, |acc, x| acc * my_ticket[x] as u128);

    println!("Part 2: {}", result);
}

pub fn run() {
    let input = load_input("src/solutions/16/data.txt");
    let data = parse_input(&input);
    let invalid_tickets = part_one(&data);
    part_two(&data, &invalid_tickets);
}
