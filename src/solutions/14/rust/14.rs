use itertools::Itertools;
use std::collections::HashMap;

use crate::lib::input::load_input;

#[derive(Debug, Clone)]
enum Instruction {
    Mask(Vec<Option<bool>>),
    Write(usize, u64),
}

fn parse_input(input: &String) -> Vec<Instruction> {
    let instructions = input
        .lines()
        .map(|l| {
            let (a, b) = l.split(" = ").next_tuple().unwrap();
            if a == "mask" {
                let value = b
                    .chars()
                    .map(|c| match c {
                        '1' => Some(true),
                        '0' => Some(false),
                        'X' => None,
                        _ => unreachable!(),
                    })
                    .collect();
                return Instruction::Mask(value);
            } else {
                let (a, b) = l.split("] = ").next_tuple().unwrap();
                let m = a.replace("mem[", "").parse().unwrap();
                let val = b.parse().unwrap();
                return Instruction::Write(m, val);
            }
        })
        .collect();
    // println!("{:?}", instructions);
    return instructions;
}

fn part_one(instructions: &Vec<Instruction>) {
    let mut instructions = instructions.clone();
    let mut memory = HashMap::new();
    let mut mask = vec![];
    for instr in instructions {
        match instr {
            Instruction::Mask(val) => mask = val,
            Instruction::Write(addr, mut val) => {
                for (i, opt) in mask.iter().rev().enumerate() {
                    match opt {
                        Some(true) => {
                            let bits = 1 << i;
                            val |= bits;
                        }
                        Some(false) => {
                            let bits = !(1 << i);
                            val &= bits;
                        }
                        None => {}
                    }
                }
                memory.insert(addr, val);
            }
            _ => panic!("unsupported Instruction"),
        }
    }
    println!("{}", memory.values().sum::<u64>());
}

pub fn run() {
    let input = load_input("src/solutions/14/data.txt");
    let instructions = parse_input(&input);
    part_one(&instructions);
}
