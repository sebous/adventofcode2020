use std::collections::HashMap;

fn calculate_nth_turn(input: &Vec<u64>, n: u64) {
    let mut last: u64 = 0;
    let mut turn: u64 = 1;
    let mut cache: HashMap<u64, (u64, u64)> = HashMap::new();

    loop {
        if (turn as usize) - 1 < input.len() {
            let i = (turn as usize) - 1;
            last = input[i];
        } else {
            let (t1, t2) = cache.get(&last).unwrap();
            if *t2 == 0 {
                last = 0;
            } else {
                last = *t1 - *t2;
            }
        }

        // println!("{}, {}", turn, last);
        if let Some(turns) = cache.get_mut(&last) {
            let (t1, t2) = turns;
            *t2 = *t1;
            *t1 = turn;
        } else {
            cache.insert(last, (turn, 0));
        }

        turn += 1;

        if turn == n + 1 {
            println!("{}", last);
            break;
        }
    }
}

pub fn run() {
    let input = vec![13, 0, 10, 12, 1, 5, 8];
    calculate_nth_turn(&input, 2020);
    calculate_nth_turn(&input, 30000000);
}
