use std::collections::HashSet;

use crate::lib::input::load_input;

type ActiveSet = HashSet<(isize, isize, isize)>;
type ActiveSet4d = HashSet<Coord4d>;
type Coord = (isize, isize, isize);
type Coord4d = (isize, isize, isize, isize);
type Limits = (isize, isize, isize, isize, isize, isize);
type Limits4d = (isize, isize, isize, isize, isize, isize, isize, isize);

fn parse_input(input: &String) -> ActiveSet {
    let mut active_set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                active_set.insert((x as isize, y as isize, 0));
            }
        }
    }
    return active_set;
}

#[allow(dead_code)]
fn pretty_print(active_set: &ActiveSet, generation: u32) {
    let (min_x, min_y, min_z, max_x, max_y, max_z) = get_min_max(&active_set);
    println!("=== GEN: {} ===", generation);
    for z in min_z..max_z + 1 {
        println!("z={}", z);
        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
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
                if *pos == (0, 1, 0) {
                    // println!("here");
                }
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
    let mut source_set = active_set_starting.clone();
    let mut next_set = source_set.clone();
    for _ in 0..cycles {
        // pretty_print(&source_set, i);
        let (min_x, min_y, min_z, max_x, max_y, max_z) = get_min_max(&source_set);

        for x in min_x - 1..max_x + 2 {
            for y in min_y - 1..max_y + 2 {
                for z in min_z - 1..max_z + 2 {
                    let active_neib_count = count_active_neighbours(&source_set, &(x, y, z));
                    if let Some(_) = &source_set.get(&(x, y, z)) {
                        if active_neib_count != 2 && active_neib_count != 3 {
                            &next_set.remove(&(x, y, z));
                        }
                    } else {
                        if active_neib_count == 3 {
                            &next_set.insert((x, y, z));
                        }
                    }
                }
            }
        }
        source_set = next_set.clone();
    }

    println!("Part 1: {}", next_set.len());
}

fn parse_input_4d(input: &String) -> ActiveSet4d {
    let mut active_set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                active_set.insert((x as isize, y as isize, 0, 0));
            }
        }
    }
    return active_set;
}

fn get_min_max_4d(active_set: &ActiveSet4d) -> Limits4d {
    let min_x = active_set.iter().map(|(x, _, _, _)| *x).min().unwrap();
    let min_y = active_set.iter().map(|(_, y, _, _)| *y).min().unwrap();
    let min_z = active_set.iter().map(|(_, _, z, _)| *z).min().unwrap();
    let min_w = active_set.iter().map(|(_, _, _, w)| *w).min().unwrap();
    let max_x = active_set.iter().map(|(x, _, _, _)| *x).max().unwrap();
    let max_y = active_set.iter().map(|(_, y, _, _)| *y).max().unwrap();
    let max_z = active_set.iter().map(|(_, _, z, _)| *z).max().unwrap();
    let max_w = active_set.iter().map(|(_, _, _, w)| *w).max().unwrap();
    return (min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w);
}

fn count_active_neighbours_4d(active_set: &ActiveSet4d, pos: &Coord4d) -> i32 {
    let mut counter = 0;
    let (x, y, z, w) = *pos;
    for nx in x - 1..x + 2 {
        for ny in y - 1..y + 2 {
            for nz in z - 1..z + 2 {
                for nw in w - 1..w + 2 {
                    if (nx, ny, nz, nw) == (x, y, z, w) {
                        continue;
                    }
                    if let Some(_) = active_set.get(&(nx, ny, nz, nw)) {
                        counter += 1;
                    }
                }
            }
        }
    }
    // println!("{}: {:?}", counter, (x, y, z, w));
    return counter;
}

fn calculate_cycles_4d(active_set_starting: &ActiveSet4d, cycles: u32) {
    let mut source_set = active_set_starting.clone();
    let mut next_set = source_set.clone();
    for _ in 0..cycles {
        // pretty_print(&source_set, i);
        let (min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w) = get_min_max_4d(&source_set);

        for x in min_x - 1..max_x + 2 {
            for y in min_y - 1..max_y + 2 {
                for z in min_z - 1..max_z + 2 {
                    for w in min_w - 1..max_w + 2 {
                        let active_neib_count =
                            count_active_neighbours_4d(&source_set, &(x, y, z, w));
                        if let Some(_) = &source_set.get(&(x, y, z, w)) {
                            if active_neib_count != 2 && active_neib_count != 3 {
                                &next_set.remove(&(x, y, z, w));
                            }
                        } else {
                            if active_neib_count == 3 {
                                &next_set.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        source_set = next_set.clone();
    }

    println!("Part 2: {}", next_set.len());
}

pub fn run() {
    let input = load_input("src/solutions/17/data.txt");
    let active_set_starting = parse_input(&input);
    calculate_cycles(&active_set_starting, 6);
    let active_starting_4d = parse_input_4d(&input);
    calculate_cycles_4d(&active_starting_4d, 6);
}
