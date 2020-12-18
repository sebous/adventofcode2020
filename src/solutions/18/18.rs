use crate::lib::input::load_input;
use std::char;
#[derive(PartialEq, Debug)]
enum Operator {
    NONE,
    ADD,
    MULTIPLY,
}

fn detect_nested_exp(slice: &[char]) -> (&[char], usize) {
    let mut opening_count = 0;
    let mut closing_count = 0;
    for (i, char) in slice.iter().enumerate() {
        match *char {
            '(' => opening_count += 1,
            ')' => {
                if opening_count == closing_count {
                    return (&slice[..i], i);
                } else {
                    closing_count += 1;
                }
            }
            _ => {}
        }
    }
    return (&slice[..], 0);
}

// fn detect_nested_str(slice: Vec<&str>) -> (Vec<String>, usize) {
//     let mut opening_count = 0;
//     let mut closing_count = 0;
//     for (i, str) in slice.iter().enumerate() {
//         match *str {
//             "(" => opening_count += 1,
//             ")" => {
//                 if opening_count == closing_count {
//                     return (
//                         slice[..i]
//                             .iter()
//                             .map(|s| s.to_string())
//                             .collect::<Vec<String>>(),
//                         i,
//                     );
//                 } else {
//                     closing_count += 1;
//                 }
//             }
//             _ => {}
//         }
//     }
//     return (
//         slice[..]
//             .iter()
//             .map(|s| s.to_string())
//             .collect::<Vec<String>>(),
//         0,
//     );
// }

fn calculate_expression(exp: &[char]) -> u64 {
    let mut operator = Operator::NONE;
    let mut sum = 0;
    let mut i = 0;

    loop {
        if i == exp.len() {
            break;
        }

        let char = exp[i];
        if char.is_digit(10) {
            let digit = char.to_digit(10).unwrap() as u64;
            match operator {
                Operator::NONE => sum = digit,
                Operator::ADD => sum += digit,
                Operator::MULTIPLY => sum *= digit,
            }
        } else if char == '+' {
            operator = Operator::ADD;
        } else if char == '*' {
            operator = Operator::MULTIPLY;
        } else if char == '(' {
            let (nested_exp, end_index) = detect_nested_exp(&exp[i + 1..]);
            let nested_sum = calculate_expression(nested_exp);
            match operator {
                Operator::ADD => sum += nested_sum,
                Operator::MULTIPLY => sum *= nested_sum,
                Operator::NONE => sum = nested_sum,
            }
            i += end_index;
        }
        i += 1;
    }
    // println!("{}", sum);
    sum
}

// fn calculate_additions(exp: Vec<String>) -> Vec<String> {
//     let mut operator = Operator::NONE;
//     let mut i = 0;
//     let mut buffer: Vec<String> = vec![];

//     let mut prev_digit = 0;
//     loop {
//         if i == exp.len() {
//             break;
//         }
//         let char = &exp[i];
//         if char.parse::<u64>().is_ok() {
//             let digit = char.parse::<u64>().unwrap();
//             match operator {
//                 Operator::NONE => prev_digit = digit,
//                 Operator::MULTIPLY => {
//                     buffer.push(prev_digit.to_string());
//                     buffer.push("*".to_string());
//                     prev_digit = digit;
//                 }
//                 Operator::ADD => prev_digit += digit,
//             }
//         } else if char == "+" {
//             operator = Operator::ADD;
//         } else if char == "*" {
//             operator = Operator::MULTIPLY;
//         } else if char == "(" {
//             let (nested_exp, end_index) = detect_nested_str(
//                 exp[i + 1..]
//                     .iter()
//                     .map(|s| s.as_str())
//                     .collect::<Vec<&str>>(),
//             );
//             let mut nested = calculate_additions(nested_exp);
//             buffer.push("(".to_string());
//             buffer.append(&mut nested);
//             buffer.push(")".to_string());
//             i += end_index;
//         }
//         i += 1;
//     }
//     return buffer;
// }

// fn calculate_rest(exp: Vec<&str>) -> u64 {
//     let mut operator = Operator::NONE;
//     let mut sum = 0;
//     let mut i = 0;

//     loop {
//         if i == exp.len() {
//             break;
//         }

//         let char = exp[i];
//         if char.parse::<u64>().is_ok() {
//             let digit = char.parse::<u64>().unwrap();
//             match operator {
//                 Operator::NONE => sum = digit,
//                 Operator::ADD => sum += digit,
//                 Operator::MULTIPLY => sum *= digit,
//             }
//         } else if char == "+" {
//             operator = Operator::ADD;
//         } else if char == "*" {
//             operator = Operator::MULTIPLY;
//         } else if char == "(" {
//             let (nested_exp, end_index) = detect_nested_str(exp[i + 1..].to_vec());
//             let nested_sum =
//                 calculate_rest(nested_exp.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
//             match operator {
//                 Operator::ADD => sum += nested_sum,
//                 Operator::MULTIPLY => sum *= nested_sum,
//                 Operator::NONE => sum = nested_sum,
//             }
//             i += end_index;
//         }
//         i += 1;
//     }
//     println!("{}", sum);
//     sum
// }

fn part_one(lines: &Vec<String>) {
    let sum = lines
        .iter()
        .map(|line| calculate_expression(&line.chars().collect::<Vec<char>>()[..]))
        .map(|n| n as u128)
        .sum::<u128>();
    // let exp = lines[0].chars().collect::<Vec<char>>();
    // let sum = calculate_expression(&exp[..]);
    println!("{}", sum);
}

// fn part_two(lines: &Vec<String>) {
//     let exp = lines[0]
//         .chars()
//         .map(|c| c.to_string())
//         .collect::<Vec<String>>();
//     let p = calculate_additions(exp);
//     calculate_rest(p.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
// }

#[allow(dead_code)]
pub fn run() {
    let input = load_input("src/solutions/18/data.txt")
        .lines()
        .map(|l| l.replace(" ", "").to_owned())
        .collect::<Vec<String>>();
    part_one(&input);
    // part_two(&input);
}
