use std::env::args;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_use]
mod shared;

define_solvers!(SOLVERS, day1, day2, day3, day4, day5, day6, day7, day8, day9);

fn main() {
    let args: Vec<String> = args().collect();

    let day: usize = args[1]
        .parse()
        .expect("day should be a number from 1 to 25");

    let part: usize = args[2].parse().expect("part should be equal to 1 or 2");

    let test = args
        .get(3)
        .map(|arg| arg.parse::<u8>().expect("test should be equal to 0 or 1"))
        .unwrap_or(0);

    let use_test_data = test == 1;
    let filename = if use_test_data { "test" } else { "input" };

    let day_input_folder = format!("day{day}");
    let input = shared::read_input(&day_input_folder, filename);
    let solver = SOLVERS[day - 1][part - 1];

    let result = solver(&input);

    let data_type = if use_test_data { "test" } else { "real" };

    println!("Result for day {day} part {part} ({data_type} data): {result}");
}
