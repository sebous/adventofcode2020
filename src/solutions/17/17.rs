use std::collections::HashSet;

use crate::lib::input::load_input;

type ActiveSet = HashSet<(isize, isize, isize)>;
type Coord = (isize, isize, isize);
type Limits = (isize, isize, isize, isize, isize, isize);

fn parse_input(input: &String) -> ActiveSet {
    let mut active_set = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                active_set.insert((x as isize, y as isize, 0));
            }
        }
    }
    return active_set;
}

fn pretty_print(active_set: &ActiveSet) {
    let (min_x, min_y, min_z, max_x, max_y, max_z) = get_min_max(&active_set);
    for z in min_z..max_z + 1 {
        println!("z={}", z);
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                if let Some(_) = &active_set.get(&(x, y, z)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn get_min_max(active_set: &ActiveSet) -> Limits {
    let min_x = active_set.iter().map(|(x, _, _)| *x).min().unwrap();
    let min_y = active_set.iter().map(|(_, y, _)| *y).min().unwrap();
    let min_z = active_set.iter().map(|(_, _, z)| *z).min().unwrap();
    let max_x = active_set.iter().map(|(x, _, _)| *x).max().unwrap();
    let max_y = active_set.iter().map(|(_, y, _)| *y).max().unwrap();
    let max_z = active_set.iter().map(|(_, _, z)| *z).max().unwrap();
    return (min_x, min_y, min_z, max_x, max_y, max_z);
}

fn count_active_neighbours(active_set: &ActiveSet, pos: &Coord) -> i32 {
    let mut counter = 0;
    let (x, y, z) = *pos;
    for nx in x - 1..x + 2 {
        for ny in y - 1..y + 2 {
            for nz in z - 1..z + 2 {
                if (nx, ny, nz) == (x, y, z) {
                    continue;
                }
                if let Some(_) = active_set.get(&(nx, ny, nz)) {
                    counter += 1;
                }
            }
        }
    }
    return counter;
}

fn calculate_cycles(active_set_starting: &ActiveSet, cycles: u32) {
    let mut active_set = active_set_starting.clone();
    for _ in 0..cycles {
        pretty_print(&active_set);
        let (min_x, min_y, min_z, max_x, max_y, max_z) = get_min_max(&active_set);

        for x in min_x - 1..max_x + 2 {
            for y in min_y - 1..max_y + 2 {
                for z in min_z - 1..max_z + 2 {
                    let active_neib_count = count_active_neighbours(&active_set, &(x, y, z));
                    if let Some(_) = &active_set.get(&(x, y, z)) {
                        if active_neib_count != 2 && active_neib_count != 3 {
                            &active_set.remove(&(x, y, z));
                        }
                    } else {
                        if active_neib_count == 3 {
                            &active_set.insert((x, y, z));
                        }
                    }
                }
            }
        }
    }
    println!("{}: {:?}", active_set.len(), active_set);
}

pub fn run() {
    let input = load_input("src/solutions/17/data.txt");
    let active_set_starting = parse_input(&input);
    calculate_cycles(&active_set_starting, 3);
}

for line in file {
    for char in line {

    }
}
