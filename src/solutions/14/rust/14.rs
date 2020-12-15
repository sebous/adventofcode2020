use itertools::Itertools;
use std::collections::HashMap;

use crate::lib::input::load_input;

#[derive(Debug, Clone)]
enum Instruction {
    Mask(Vec<Option<bool>>),
    Write(usize, u64),
}

#[derive(Debug, Clone)]
enum InstructionV2 {
    Mask(Vec<char>),
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

fn parse_input_v2(input: &String) -> Vec<InstructionV2> {
    let instructions = input
        .lines()
        .map(|l| {
            let (a, b) = l.split(" = ").next_tuple().unwrap();
            if a == "mask" {
                let value = b.chars().into_iter().collect();
                return InstructionV2::Mask(value);
            } else {
                let (a, b) = l.split("] = ").next_tuple().unwrap();
                let m = a.replace("mem[", "").parse().unwrap();
                let val = b.parse().unwrap();
                return InstructionV2::Write(m, val);
            }
        })
        .collect();
    // println!("{:?}", instructions);
    return instructions;
}

fn part_one(instructions: &Vec<Instruction>) {
    let mut memory = HashMap::new();
    let mut mask = vec![];
    for instr in instructions {
        match instr {
            Instruction::Mask(val) => mask = val.clone(),
            Instruction::Write(addr, mut val) => {
                for (i, opt) in mask.iter().rev().enumerate() {
                    match opt {
                        Some(true) => {
                            val |= 1 << i;
                        }
                        Some(false) => val &= !(1 << i),
                        None => {}
                    }
                }
                memory.insert(addr, val);
            }
        }
    }
    println!("Part 1: {}", memory.values().sum::<u64>());
}

fn find_floating_permutations(addr_list: &mut Vec<usize>, addr: usize, floating_bits: &[usize]) {
    if floating_bits.len() > 0 {
        let (first, rest) = floating_bits.split_first().unwrap();
        let addr_0 = addr & !(1 << first);
        let addr_1 = addr | 1 << first;
        addr_list.push(addr_0);
        addr_list.push(addr_1);
        if rest.len() > 0 {
            find_floating_permutations(addr_list, addr_0, rest);
            find_floating_permutations(addr_list, addr_1, rest);
        }
    }
}

fn part_two(instructions: &Vec<InstructionV2>) {
    let mut memory = HashMap::new();
    let mut mask = vec![];
    for instr in instructions {
        match instr {
            InstructionV2::Mask(val) => mask = val.clone(),
            InstructionV2::Write(mut addr, val) => {
                let mut addr_list = vec![];
                let mut floating_bits = vec![];
                for (i, bit) in mask.iter().rev().enumerate() {
                    match bit {
                        '1' => addr |= 1 << i,
                        'X' => {
                            floating_bits.push(i);
                        }
                        _ => {}
                    }
                }
                find_floating_permutations(&mut addr_list, addr, &floating_bits[..]);
                let addr_list_uniq: Vec<usize> = addr_list
                    .clone()
                    .iter()
                    .unique()
                    .map(|n| n.to_owned())
                    .collect();

                // println!("{:?}", addr_list_uniq);
                for a in addr_list_uniq {
                    memory.insert(a, *val);
                }
            }
        }
    }
    println!("Part 2: {}", memory.values().sum::<u64>());
}

pub fn run() {
    let input = load_input("src/solutions/14/data.txt");
    let instructions = parse_input(&input);
    part_one(&instructions);
    part_two(&parse_input_v2(&input));
}
