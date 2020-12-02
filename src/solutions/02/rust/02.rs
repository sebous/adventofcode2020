use crate::lib::input::load_input;

struct PasswordInfo {
    pub min: u32,
    pub max: u32,
    pub letter: char,
    pub password: String,
}

fn validate_p1(pass_info: &PasswordInfo) -> bool {
    let letter_count = pass_info.password.chars().filter(|&ch| ch == pass_info.letter).count() as u32;
    if letter_count >= pass_info.min && letter_count <= pass_info.max {
        return true;
    }
    return false;
}

fn validate_p2(pass_info: &PasswordInfo) -> bool {
    let first_char = pass_info.password.chars().nth((pass_info.min - 1) as usize).unwrap();
    let second_char = pass_info.password.chars().nth((pass_info.max - 1) as usize).unwrap();
    let search_char = pass_info.letter;

    if first_char == search_char && second_char != search_char {
        return true;
    }
    if first_char != search_char && second_char == search_char {
        return true;
    }
    return false;
}

pub fn run() {
    let input = load_input("src/solutions/02/data.txt").lines().map(|l| {
        let splitted_line = l.split(" ").collect::<Vec<&str>>();

        PasswordInfo {
            min: splitted_line[0].split("-").collect::<Vec<&str>>()[0].parse().unwrap(),
            max: splitted_line[0].split("-").collect::<Vec<&str>>()[1].parse().unwrap(),
            letter: splitted_line[1].chars().next().unwrap(),
            password: String::from(splitted_line[2]),
        }
    }).collect::<Vec<PasswordInfo>>();


    println!("Day 2:");
    let part1 = input.as_slice().clone().into_iter().filter(|inf| validate_p1(inf)).count();
    println!("Part 1: {}", part1);

    let part2 = input.as_slice().clone().into_iter().filter(|inf| validate_p2(inf)).count();
    println!("Part 2: {}", part2);
}