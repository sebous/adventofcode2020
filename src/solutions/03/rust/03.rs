use crate::lib::input::load_input;
use std::collections::HashMap;

type Coord = (u32, u32);
type Grid = HashMap<Coord, bool>;
type GridInfo = (Grid, u32, u32);

fn create_source_grid(input: String) -> GridInfo {
    let mut source_height: u32 = 0;
    let mut source_width: u32 = 0;
    let mut grid: Grid = Grid::new();

    for (y, line) in input.lines().enumerate() {
        if (y as u32) + 1 > source_height {
            source_height = (y as u32) + 1;
        }
        for (x, spot) in line.split("").enumerate() {
            if (x as u32) + 1 > source_width {
                source_width = (x as u32) + 1;
            }
            let spot_value = if spot == "#" { true } else { false };
            grid.insert((x as u32, y as u32), spot_value);
        }
    }

    return (grid, source_width, source_height);
}

fn count_slope(slope: (u32, u32), source_grid_info: GridInfo) {
    let trees_count: u32 = 0;
    let current_pos: Coord = (0, 0);
}

pub fn run() {
    let input = load_input("src/solutions/03/data.txt");
    let grid_info = create_source_grid(input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
}
