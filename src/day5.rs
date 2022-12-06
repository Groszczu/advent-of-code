use std::str::FromStr;

use crate::{shared::PuzzleResult, test_puzzle};

mod crane;

use crane::{Crane, CrateMover, MoveInstruction};

pub fn part1(input: &str) -> PuzzleResult {
    let (mut crane, move_instructions) = transform_input(input);

    println!("{:?}", crane);

    for move_instruction in move_instructions {
        crane.move_containers(&move_instruction, CrateMover::V9000);
    }

    get_result(&crane)
}

pub fn part2(input: &str) -> PuzzleResult {
    let (mut crane, move_instructions) = transform_input(input);

    for move_instruction in move_instructions {
        crane.move_containers(&move_instruction, CrateMover::V9001);
    }

    get_result(&crane)
}

fn transform_input(input: &str) -> (Crane, Vec<MoveInstruction>) {
    let input_parts: Vec<_> = input.split("\n\n").collect();
    let start_position = input_parts[0];
    let move_instructions = input_parts[1];

    let crane = Crane::from_str(start_position).expect("start position should be valid");

    let move_instructions: Vec<_> = move_instructions
        .lines()
        .map(|instruction| {
            instruction
                .split(' ')
                .filter_map(|part| part.parse::<usize>().ok())
                .collect::<MoveInstruction>()
        })
        .collect();

    (crane, move_instructions)
}

fn get_result(crane: &Crane) -> PuzzleResult {
    crane
        .top_containers()
        .map(|container| *container.expect("all top containers should exist"))
        .map(|container| container.label())
        .collect::<String>()
        .into()
}

test_puzzle!(5, "CMZ".to_string(), "MCD".to_string());
