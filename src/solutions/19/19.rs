use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, str::Lines};

use crate::lib::input::load_input;

type Rules = HashMap<String, String>;

fn parse_input(input: &String) -> (Rules, Lines) {
    let mut rules: Rules =
        input
            .split("\n\n")
            .next()
            .unwrap()
            .lines()
            .fold(HashMap::new(), |mut acc, rule| {
                let (id, rule_rest) = rule.split(": ").next_tuple().unwrap();
                acc.insert(id.to_owned(), rule_rest.replace('"', ""));
                return acc;
            });
    let messages = input.split("\n\n").nth(1).unwrap().lines();

    // println!("{:?}", rules);
    return (rules, messages);
}

fn calculate_rules(rules: &Rules, from_rule: String) -> String {
    let rule_line = rules.get(&from_rule).unwrap();
    if !rule_line.chars().next().unwrap().is_alphabetic() {
        let regex = rule_line
            .split(" | ")
            .map(|b| {
                format!(
                    "{}|",
                    b.split(" ")
                        .map(|rule| calculate_rules(rules, rule.to_owned()))
                        .collect::<String>()
                )
            })
            .collect::<String>();
        return format!("({})", &regex[..regex.len() - 1]);
    } else {
        return rule_line.to_owned();
    }
}

fn validate(messages: &Lines, rules: &Rules) {
    let regex = format!("^{}$", calculate_rules(rules, "0".to_owned()));
    let regex = Regex::new(&regex).unwrap();
    println!("regex: {}", regex.to_string());
    let valid_count = messages.clone().filter(|msg| regex.is_match(msg)).count();
    println!("Part 1: {}", valid_count);
}

pub fn run() {
    let input = load_input("src/solutions/19/data.txt");
    let (rules, messages) = parse_input(&input);
    validate(&messages, &rules);
}
