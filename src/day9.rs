mod rope;

use std::str::FromStr;

use crate::{shared::PuzzleResult, test_solvers};

use rope::{Motion, Rope};

pub fn part1(input: &str) -> PuzzleResult {
    let motions = transform_input(input);
    let mut rope = Rope::new(2);

    for motion in &motions {
        rope.move_head(*motion);
    }

    let result = rope.tail_positions().len() as i64;
    result.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let motions = transform_input(input);
    let mut rope = Rope::new(10);

    for motion in &motions {
        rope.move_head(*motion);
    }

    let result = rope.tail_positions().len() as i64;
    result.into()
}

fn transform_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .filter_map(|line| Motion::from_str(line).ok())
        .collect::<Vec<_>>()
}

test_solvers!(13, 1);
