#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;
extern crate time;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
mod day8;

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

    if cfg!(feature = "day4") {
        println!("Day4, part1: {}", day4::solve_part1("data/day4/input.txt"));
        println!("Day4, part2: {}", day4::solve_part2("data/day4/input.txt"));
    }

    if cfg!(feature = "day5") {
        println!("Day5, part1: {}", day5::solve_part1("data/day5/input.txt"));
        println!("Day5, part2: {}", day5::solve_part2("data/day5/input.txt"));
    }

    if cfg!(feature = "day7") {
        println!("Day7, part1: {}", day7::solve_part1("data/day7/input.txt"));
        println!("Day7, part2: {}", day7::solve_part2("data/day7/input.txt"));
    }

    if cfg!(feature = "day8") {
        println!("Day8, example part1: {}", day8::solve_part1("data/day8/example.txt"));
        println!("Day8, part1: {}", day8::solve_part1("data/day8/input.txt"));

        println!("Day8, example part2: {}", day8::solve_part2("data/day8/example.txt"));
        println!("Day8, part2: {}", day8::solve_part2("data/day8/input.txt"));
    }
}
