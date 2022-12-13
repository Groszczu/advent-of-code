use std::collections::HashSet;

use crate::{shared::PuzzleResult, test_solvers};

pub fn part1(input: &str) -> PuzzleResult {
    chars_precessed_to_unique_sequence(input, 4)
}

pub fn part2(input: &str) -> PuzzleResult {
    chars_precessed_to_unique_sequence(input, 14)
}

fn chars_precessed_to_unique_sequence(input: &str, sequence_size: usize) -> PuzzleResult {
    let chars = input.chars().collect::<Vec<_>>();

    let (index, _) =
        find_unique_sequence(&chars, sequence_size).expect("unique sequence should exist");

    let chars_processed = (index + sequence_size) as i64;

    chars_processed.into()
}

fn find_unique_sequence(chars: &Vec<char>, sequence_size: usize) -> Option<(usize, &[char])> {
    chars
        .windows(sequence_size)
        .enumerate()
        .find(|(_, sequence)| {
            let mut unique_chars = HashSet::new();
            sequence.iter().all(move |&c| unique_chars.insert(c))
        })
}

test_solvers!(7, 19);
