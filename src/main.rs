use aoc_2019::days::problem::Problem;
use aoc_2019::days::*;
use std::env;

fn main() {
    let day = parse_arg();
    let problem: Box<dyn Problem> = match day {
        1 => Box::new(Day1 {}),
        2 => Box::new(Day2 {}),
        3 => Box::new(Day3 {}),
        4 => Box::new(Day4 {}),
        _ => unreachable!("Day must be between 1 and 25 inclusive"),
    };

    println!("Part 1 result: {}", problem.part_one());
    println!("Part 2 result: {}", problem.part_two());
}

/// parse day argument, should be a number between 1 and 25 inclusive
fn parse_arg() -> usize {
    let args: Vec<String> = env::args().collect();
    let incorrect_arg_count_msg =
        "Incorrect number of argument; only argument required is day number";
    let invalid_day_msg = "Day must be integer between 1 and 25 for advent-of-code";

    if args.len() != 2 {
        panic!("{}", incorrect_arg_count_msg);
    }
    let day = str::parse::<usize>(&args[1]).expect(invalid_day_msg);
    if !(1..=25).contains(&day) {
        panic!("{}", invalid_day_msg);
    }
    day
}
