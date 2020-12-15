use crate::lib::input::load_input;

fn parse_input(input: &String) -> (u32, Vec<u32>) {
    let mut lines = input.lines();
    let timestamp: u32 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&str| str != "x")
        .map(|str| str.parse().unwrap())
        .collect();
    return (timestamp, buses);
}

fn parse_input_w_x(input: &String) -> Vec<u32> {
    let mut lines = input.lines().skip(1);
    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|str| if str == "x" { 0 } else { str.parse().unwrap() })
        .collect();
    println!("{:?}", buses);
    return buses;
}

fn part_one(data: &(u32, Vec<u32>)) {
    let (timestamp, buses) = data;
    let mut time = timestamp.clone();
    let result_id: u32;

    'outer: loop {
        for id in buses.iter() {
            if time % *id == 0 {
                result_id = *id;
                break 'outer;
            }
        }
        time += 1;
    }
    println!("Part 1: {}", result_id * (time - timestamp));
}

// fn part_two(buses: &Vec<u32>) {}

pub fn run() {
    let input = load_input("src/solutions/13/data.txt");
    let data = parse_input(&input);
    part_one(&data);
    let _buses_w_x = parse_input_w_x(&input);
    // part_two(&buses_w_x);
}
