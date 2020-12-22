use std::collections::VecDeque;

use crate::lib::input::load_input;

struct Game {
    player1_deck: VecDeque<u32>,
    player2_deck: VecDeque<u32>,
}

impl Game {
    pub fn new(p1_deck: Vec<u32>, p2_deck: Vec<u32>) -> Game {
        Game {
            player1_deck: VecDeque::from(p1_deck),
            player2_deck: VecDeque::from(p2_deck),
        }
    }
    fn play_until_win(&mut self) {
        loop {
            self.play_round();
            if self.check_if_win() {
                break;
            }
        }
    }
    fn play_round(&mut self) {
        let p1_card = self.player1_deck[0];
        let p2_card = self.player2_deck[0];
        self.player1_deck.pop_front();
        self.player2_deck.pop_front();
        if p1_card > p2_card {
            self.player1_deck.push_back(p1_card);
            self.player1_deck.push_back(p2_card);
        } else {
            self.player2_deck.push_back(p2_card);
            self.player2_deck.push_back(p1_card);
        }
    }
    fn check_if_win(&self) -> bool {
        if self.player1_deck.len() == 0 || self.player2_deck.len() == 0 {
            return true;
        }
        false
    }
    fn calculate_score(&self) -> u32 {
        let winning_deck = if self.player1_deck.len() > self.player2_deck.len() {
            &self.player1_deck
        } else {
            &self.player2_deck
        };
        let mut score = 0;
        for (i, card) in winning_deck.iter().rev().enumerate() {
            score += card * (i as u32 + 1);
        }
        score
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let decks = input
        .split("\n\n")
        .map(|d| {
            let mut c_lines = d.lines();
            c_lines.next();
            c_lines
                .map(|l| l.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (decks[0].clone(), decks[1].clone())
}

pub fn run() {
    let input = load_input("src/solutions/22/data.txt");
    let (deck_1, deck_2) = parse_input(&input);
    let mut game = Game::new(deck_1, deck_2);
    game.play_until_win();
    println!("Part 1: {}", game.calculate_score());
}
