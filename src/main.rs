#[macro_use] extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod day3;

mod utils;

fn main() {
    if cfg!(feature = "day1") {
        println!("Day1, part1: {}", day1::solve_part1("data/day1/input.txt"));
        println!("Day1, part2: {:?}", day1::solve_part2("data/day1/input.txt"));
    }

    if cfg!(feature = "day2") {
        println!("Day2, part1: {}", day2::solve_part1("data/day2/input.txt"));
        println!("Day2, part2: {:?}", day2::solve_part2("data/day2/input.txt"));
    }

    if cfg!(feature = "day3") {
        println!("Day3, part1: {}", day3::solve_part1("data/day3/input.txt"));
        println!("Day3, part1: {}", day3::solve_part2("data/day3/input.txt"));
    }
}
