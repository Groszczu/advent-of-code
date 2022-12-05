use crate::{shared::PuzzleResult, test_puzzle};

pub fn part1(input: &str) -> PuzzleResult {
    let (mut stacks, instructions) = transform_input(input);

    for instruction in instructions {
        let (quantity, from, to) = instruction;

        let from_index = (from - 1) as usize;
        let to_index = (to - 1) as usize;
        let from_len = stacks[from_index].len();
        let crates_to_move = stacks[from_index].split_off(from_len - quantity as usize);
        stacks[to_index].extend(crates_to_move.iter().rev());
    }

    get_result(&stacks)
}

pub fn part2(input: &str) -> PuzzleResult {
    let (mut stacks, instructions) = transform_input(input);

    for instruction in instructions {
        let (quantity, from, to) = instruction;

        let from_index = (from - 1) as usize;
        let to_index = (to - 1) as usize;
        let from_len = stacks[from_index].len();
        let crates_to_move = stacks[from_index].split_off(from_len - quantity as usize);
        stacks[to_index].extend(crates_to_move.iter());
    }

    get_result(&stacks)
}

fn transform_input(input: &str) -> (Vec<Vec<&str>>, Vec<(i32, i32, i32)>) {
    let input_parts: Vec<_> = input.split("\n\n").collect();
    let start_position = input_parts[0];
    let move_instructions = input_parts[1];

    let start_position: Vec<_> = start_position
        .lines()
        .map(|crates| crates.split(' ').collect::<Vec<&str>>())
        .collect();

    let move_instructions: Vec<_> = move_instructions
        .lines()
        .map(|instruction| {
            let instruction_parts = instruction
                .split(' ')
                .map(|part| {
                    part.parse::<i32>()
                        .expect("instruction should contain numbers")
                })
                .collect::<Vec<i32>>();

            (
                instruction_parts[0],
                instruction_parts[1],
                instruction_parts[2],
            )
        })
        .collect();

    (start_position, move_instructions)
}

fn get_result(stacks: &Vec<Vec<&str>>) -> PuzzleResult {
    stacks
        .iter()
        .map(|stack| *stack.last().expect("crate stack should have an element"))
        .collect::<Vec<_>>()
        .join("")
        .into()
}

test_puzzle!(5, "CMZ".to_string(), "MCD".to_string());
