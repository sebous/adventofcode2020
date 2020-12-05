use crate::lib::input;

pub fn day01() {
    let input_data = input::load_input("src/solutions/01/data.txt");
    let data = input_data
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u32>>();

    println!("Day 1:");
    for x in 0..data.len() {
        for y in x + 1..data.len() {
            if data[x] + data[y] == 2020 {
                println!("Part 1: {}", data[x] * data[y]);
            }

            for z in y + 1..data.len() {
                if data[x] + data[y] + data[z] == 2020 {
                    println!("Part 2: {}", data[x] * data[y] * data[z]);
                }
            }
        }
    }
}
