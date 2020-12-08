use crate::lib::input::load_input;

#[derive(Debug, Clone)]
struct Instruction {
    operation: String,
    arg: i32,
    ran: bool,
}

type Program = Vec<Instruction>;

fn parse_input(input: &String) -> Program {
    let program: Vec<Instruction> = input
        .lines()
        .into_iter()
        .map(|l| {
            let mut line_halfs = l.split(" ");
            Instruction {
                operation: line_halfs.next().unwrap().to_owned(),
                arg: line_halfs.next().unwrap().parse().unwrap(),
                ran: false,
            }
        })
        .collect();
    return program;
}

fn run_till_repeat(program: &mut Program) -> (bool, i32) {
    let mut pointer: usize = 0;
    let mut accumulator: i32 = 0;

    loop {
        if pointer > program.len() - 1 {
            return (false, accumulator);
        }
        // println!(
        //     "instr: {:?}, pointer: {}, acc: {}",
        //     program[pointer], pointer, accumulator
        // );
        if program[pointer].ran {
            return (true, accumulator);
        } else {
            program[pointer].ran = true;
        }
        match program[pointer].operation.as_str() {
            "acc" => {
                accumulator += program[pointer].arg;
                pointer += 1;
            }
            "jmp" => pointer = (pointer as i32 + program[pointer].arg) as usize,
            "nop" => pointer += 1,
            _ => panic!("Panicking at pointer: {}, acc: {}", pointer, accumulator),
        }
    }
}

fn try_instr_change(program: &Program) -> i32 {
    for i in 0..program.len() - 1 {
        let mut test_program = program.clone();
        if test_program[i].operation.as_str() == "jmp" {
            test_program[i].operation = String::from("nop");
        } else if test_program[i].operation.as_str() == "nop" {
            test_program[i].operation = String::from("jmp");
        } else {
            continue;
        }

        let (stuck, accumulator) = run_till_repeat(&mut test_program);
        if !stuck {
            return accumulator;
        }
    }
    panic!("Change not found!");
}

pub fn run() {
    let input = load_input("src/solutions/08/data.txt");
    let mut program: Program = parse_input(&input);
    let (_, part1) = run_till_repeat(&mut program.clone());
    println!("Part 1: {:?}", part1);

    let part2 = try_instr_change(&mut program);
    println!("Part 2: {}", part2);
}
