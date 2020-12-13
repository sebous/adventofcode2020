use crate::lib::input::load_input;

fn part_one(instructions: &Vec<(char, i32)>) {
    let (mut x, mut y) = (0, 0);
    // 0 is north
    let mut direction: i32 = 90;

    for instr in instructions.into_iter() {
        let (ch, value) = *instr;
        match ch {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'L' => direction -= value,
            'R' => direction += value,
            'F' => match direction.rem_euclid(360) {
                0 => y += value,
                90 => x += value,
                180 => y -= value,
                270 => x -= value,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    println!("Part 1: {}", x.abs() + y.abs());
}

fn rotate(x: i32, y: i32, direction: i32) -> (i32, i32) {
    match direction {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => unreachable!(),
    }
}

fn part_two(instructions: &Vec<(char, i32)>) {
    // waypoint
    let (mut x, mut y) = (10, 1);
    // ship
    let (mut i, mut j) = (0, 0);

    for &(ch, val) in instructions.iter() {
        match ch {
            'N' => y += val,
            'S' => y -= val,
            'E' => x += val,
            'W' => x -= val,
            'L' => {
                let (x_rot, y_rot) = rotate(x, y, val);
                x = x_rot;
                y = y_rot;
            }
            'R' => {
                let (x_rot, y_rot) = rotate(x, y, 360 - val);
                x = x_rot;
                y = y_rot;
            }
            'F' => {
                i += x * val;
                j += y * val;
            }
            _ => unreachable!(),
        }
    }
    println!("Part 2: {}", i.abs() + j.abs());
}

pub fn run() {
    let input: Vec<(char, i32)> = load_input("src/solutions/12/data.txt")
        .lines()
        .into_iter()
        .map(|l| {
            let mut chars = l.chars();
            return (
                chars.next().unwrap(),
                chars.as_str().parse::<i32>().unwrap(),
            );
        })
        .collect();
    part_one(&input);
    part_two(&input);
}
