use crate::lib::input::load_input;
use std::collections::HashMap;

type Coord = (usize, usize);
type GridInfo = (HashMap<Coord, Place>, usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Place {
    EMPTY,
    OCC,
    FLOOR,
}

fn parse_input(input: &String) -> GridInfo {
    let mut grid: HashMap<Coord, Place> = HashMap::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    for (y, line) in input.lines().into_iter().enumerate() {
        if y > height {
            height = y;
        }
        for (x, char) in line.chars().enumerate() {
            let coord = (x, y);
            let place: Place;
            if char == 'L' {
                place = Place::EMPTY
            } else if char == '.' {
                place = Place::FLOOR
            } else {
                place = Place::OCC
            }
            grid.insert(coord, place);

            if x > width {
                width = x;
            }
        }
    }
    return (grid, width, height);
}

fn get_adjacent_coords(
    grid_info: &GridInfo,
    coord: &Coord,
    cache: &mut HashMap<Coord, Vec<Coord>>,
) -> Vec<Coord> {
    if let Some(adj_coords) = cache.get(coord) {
        return adj_coords.to_owned();
    }

    let (_, width, height) = grid_info;
    let (x, y) = coord;
    let min_x: usize = if x == &0 { 0 } else { x - 1 };
    let max_x: usize = if x == width { *x } else { x + 1 };
    let min_y: usize = if y == &0 { 0 } else { y - 1 };
    let max_y: usize = if y == height { *y } else { y + 1 };

    let mut adj_coords: Vec<Coord> = Vec::new();
    for target_x in min_x..max_x + 1 {
        for target_y in min_y..max_y + 1 {
            if target_x == *x && target_y == *y {
                continue;
            }
            adj_coords.push((target_x, target_y));
        }
    }

    cache.insert((*x, *y), adj_coords.to_owned());
    return adj_coords;
}

fn process_seating_layout(grid_info: &GridInfo, directional: bool) -> usize {
    let (source_grid, width, height) = grid_info;
    let mut grid = source_grid.clone();
    let mut next_grid = source_grid.clone();
    let mut cache: HashMap<Coord, Vec<Coord>> = HashMap::new();

    loop {
        let mut changes = 0;
        // 0 adjascent occupied -> opccupied
        for x in 0..*width + 1 {
            for y in 0..*height + 1 {
                let coord = (x, y);
                if let Some(place) = grid.get(&coord) {
                    if *place == Place::FLOOR || *place == Place::OCC {
                        continue;
                    }
                }

                let mut occupied = 0;
                if !directional {
                    for c in get_adjacent_coords(&grid_info, &coord, &mut cache) {
                        if let Some(place) = grid.get(&c) {
                            if *place == Place::OCC {
                                occupied += 1;
                            }
                        }
                    }
                } else {
                    occupied += count_visible_occupied(&grid, &coord);
                }

                if occupied == 0 {
                    if let Some(place) = next_grid.get_mut(&coord) {
                        *place = Place::OCC;
                        changes += 1;
                    }
                }
            }
        }

        grid = next_grid.clone();

        // 4+ adjascent occupied -> empty
        for x in 0..*width + 1 {
            for y in 0..*height + 1 {
                let coord = (x, y);
                if let Some(place) = grid.get(&coord) {
                    if *place == Place::FLOOR || *place == Place::EMPTY {
                        continue;
                    }
                }

                let mut occupied = 0;
                if !directional {
                    for c in get_adjacent_coords(&grid_info, &coord, &mut cache) {
                        if let Some(place) = grid.get(&c) {
                            if *place == Place::OCC {
                                occupied += 1;
                            }
                        }
                    }
                } else {
                    occupied += count_visible_occupied(&grid, &coord);
                }

                let occ_empty_threshold = if directional { 5 } else { 4 };

                if occupied >= occ_empty_threshold {
                    if let Some(place) = next_grid.get_mut(&coord) {
                        *place = Place::EMPTY;
                        changes += 1;
                    }
                }
            }
        }
        grid = next_grid.clone();

        // break if finished
        if changes == 0 {
            break;
        }
    }

    let occupied_seats = grid
        .iter()
        .filter(|(_, place)| **place == Place::OCC)
        .count();

    return occupied_seats;
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
fn count_visible_occupied(grid: &HashMap<Coord, Place>, coord: &Coord) -> u32 {
    let mut occupied = 0;
    let (curr_x, curr_y) = *coord;

    for d in DIRECTIONS.iter() {
        let (move_x, move_y) = *d;
        let mut prev = (curr_x, curr_y);
        loop {
            let (prev_x, prev_y) = prev;
            let next = (
                (prev_x as isize + move_x) as usize,
                (prev_y as isize + move_y) as usize,
            );
            if let Some(p) = grid.get(&next) {
                if *p == Place::OCC {
                    occupied += 1;
                    break;
                } else if *p == Place::EMPTY {
                    break;
                }
            } else {
                break;
            }
            prev = next.clone()
        }
    }
    return occupied;
}

pub fn run() {
    let input = load_input("src/solutions/11/data.txt");
    let grid_info: GridInfo = parse_input(&input);
    let part1 = process_seating_layout(&grid_info, false);
    println!("Part 1: {}", part1);

    let part2 = process_seating_layout(&grid_info, true);
    println!("Part 2: {}", part2);
}
