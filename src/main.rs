#[path = "solutions/01/rust/01.rs"]
pub mod day01;
#[path = "solutions/02/rust/02.rs"]
pub mod day02;
#[path = "solutions/05/rust/05.rs"]
pub mod day05;

#[path = "solutions/07/rust/07.rs"]
pub mod day07;
#[path = "solutions/08/rust/08.rs"]
pub mod day08;
#[path = "solutions/09/rust/09.rs"]
pub mod day09;
#[path = "solutions/10/rust/10.rs"]
pub mod day10;
#[path = "solutions/11/rust/11.rs"]
pub mod day11;
mod lib;
#[path = "solutions/test.rs"]
pub mod test;

fn main() {
    // day01::day01();
    // day02::run();
    // day05::run();
    // day07::run();
    // day08::run();
    // day09::run();
    // test::run();
    // day10::run();
    day11::run();
}
