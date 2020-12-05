use crate::lib::input::load_input;

struct SeatInfo {
    row: u32,
    column: u32,
    id: u32,
}

pub fn run() {
    let input = load_input("src/solutions/05/data.txt")
        .lines()
        .map(|l| String::from(l))
        .collect::<Vec<String>>();

    let seat_infos = input
        .iter()
        .map(|line| {
            let mut row_low = 0f32;
            let mut row_high = 127f32;
            let code = line.chars().into_iter().collect::<Vec<char>>();
            (0..7).for_each(|i| {
                if code[i] == 'F' {
                    row_high = row_low + ((row_high - row_low) / 2f32).floor();
                }
                if code[i] == 'B' {
                    row_low = row_low + ((row_high - row_low) / 2f32).ceil();
                }
            });

            let mut col_low = 0f32;
            let mut col_high = 7f32;
            (7..10).for_each(|i| {
                if code[i] == 'R' {
                    col_low = col_low + ((col_high - col_low) / 2f32).ceil();
                }
                if code[i] == 'L' {
                    col_high = col_low + ((col_high - col_low) / 2f32).floor();
                }
            });
            return SeatInfo {
                column: col_high as u32,
                row: row_high as u32,
                id: row_high as u32 * 8 + col_high as u32,
            };
        })
        .collect::<Vec<SeatInfo>>();

    let highest_id = seat_infos.iter().max_by_key(|seat| seat.id).unwrap().id;
    println!("Part 1: {}", highest_id);

    let seat_ids = seat_infos
        .iter()
        .filter(|seat| seat.row != 127 && seat.row != 0)
        .map(|s| s.id)
        .collect::<Vec<u32>>();

    let mut my_seat_id = 0;
    let ids = seat_ids.clone();
    for id in seat_ids {
        if !&ids.contains(&(id + 1)) && ids.contains(&(id + 2)) {
            my_seat_id = id + 1;
            break;
        }
        if !ids.contains(&(id - 1)) && ids.contains(&(id - 2)) {
            my_seat_id = id - 1;
            break;
        }
    }

    println!("Part 2: {}", my_seat_id);
}
