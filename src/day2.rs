use std::str::FromStr;

use crate::test_puzzle;

mod rps;

pub fn part1(input: &str) -> i64 {
    let moves = transform_input_for_part1(input);

    moves
        .iter()
        .map(|(their_move, my_move)| {
            let move_score: i64 = my_move.score().into();
            let result_score: i64 = my_move.get_result(their_move).score().into();
            (move_score + result_score) as i64
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let moves = transform_input_for_part2(input);

    moves
        .iter()
        .map(|(their_move, expected_result)| {
            let my_move = match expected_result {
                rps::GameResult::Win => their_move.loses_with(),
                rps::GameResult::Draw => their_move.draws_with(),
                rps::GameResult::Lose => their_move.wins_with(),
            };
            let move_score = my_move.score();
            let result_score = my_move.get_result(their_move).score();
            (move_score + result_score) as i64
        })
        .sum()
}

fn transform_input_for_part1(input: &str) -> Vec<(rps::Move, rps::Move)> {
    input
        .lines()
        .map(|moves| {
            let moves: Vec<&str> = moves.split(' ').collect();
            let their_move = rps::Move::from_str(moves[0]).expect("their move should be valid");
            let my_move = rps::Move::from_str(moves[1]).expect("my move should be valid");
            (their_move, my_move)
        })
        .collect()
}

fn transform_input_for_part2(input: &str) -> Vec<(rps::Move, rps::GameResult)> {
    input
        .lines()
        .map(|game| {
            let game: Vec<&str> = game.split(' ').collect();
            let their_move = rps::Move::from_str(game[0]).expect("their move should be valid");
            let expected_result =
                rps::GameResult::from_str(game[1]).expect("expected game result should be valid");
            (their_move, expected_result)
        })
        .collect()
}

test_puzzle!(2, 15, 12);
