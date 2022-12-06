use crate::{shared::PuzzleResult, test_puzzle};

mod crane;

pub fn part1(input: &str) -> PuzzleResult {
    let (mut stacks, instructions) = transform_input(input);

    for instruction in instructions {
        let (quantity, from, to) = instruction;

        let crates_to_move = pick_crates_up(&mut stacks, quantity, from);

        put_creates_down(&mut stacks, to, crates_to_move.iter().copied().rev());
    }

    get_result(&stacks)
}

pub fn part2(input: &str) -> PuzzleResult {
    let (mut stacks, instructions) = transform_input(input);

    for instruction in instructions {
        let (quantity, from, to) = instruction;

        let crates_to_move = pick_crates_up(&mut stacks, quantity, from);

        put_creates_down(&mut stacks, to, crates_to_move.iter().copied());
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
                .filter_map(|part| part.parse::<i32>().ok())
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

fn pick_crates_up<'a>(stacks: &mut Vec<Vec<&'a str>>, quantity: i32, from: i32) -> Vec<&'a str> {
    let from_index = (from - 1) as usize;
    let from_len = stacks[from_index].len();
    stacks[from_index].split_off(from_len - quantity as usize)
}

fn put_creates_down<'a>(
    stacks: &mut Vec<Vec<&'a str>>,
    to: i32,
    crates: impl Iterator<Item = &'a str>,
) -> () {
    let to_index = (to - 1) as usize;
    stacks[to_index].extend(crates);
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
