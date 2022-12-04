use std::str::FromStr;

use crate::test_puzzle;

mod cleaning;

pub fn part1(input: &str) -> i64 {
    let pairs = transform_input(input);

    pairs
        .iter()
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .collect::<Vec<_>>()
        .len() as i64
}

pub fn part2(input: &str) -> i64 {
    let pairs = transform_input(input);

    pairs
        .iter()
        .filter(|(first, second)| first.overlap(second))
        .collect::<Vec<_>>()
        .len() as i64
}

fn transform_input(input: &str) -> Vec<(cleaning::Section, cleaning::Section)> {
    input
        .lines()
        .map(|pair| {
            let assignments: Vec<_> = pair.split(",").collect();
            let first = assignments[0];
            let second = assignments[1];
            (
                cleaning::Section::from_str(first).expect("first assignment should be valid"),
                cleaning::Section::from_str(second).expect("second assignment should be valid"),
            )
        })
        .collect()
}

test_puzzle!(4, 2, 4);
