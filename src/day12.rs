mod dijkstra;

use std::str::FromStr;

use crate::{shared::PuzzleResult, test_solvers};

use dijkstra::Heightmap;

use self::dijkstra::Position;

pub fn part1(input: &str) -> PuzzleResult {
    let (start_position, end, heightmap) = transform_input_for_part_1(input);

    let start_node = heightmap
        .get(&start_position)
        .expect("start node should exist");

    let distance_from_start_to_end = heightmap.distances_to(end)[start_node] as i64;

    distance_from_start_to_end.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let (start_positions, end, heightmap) = transform_input_for_part_2(input);

    let distances_to_end = heightmap.distances_to(end);

    let min_distance = distances_to_end
        .iter()
        .filter(|(&node, _distance)| start_positions.contains(&node.position()))
        .map(|(_node, distance)| distance)
        .min()
        .unwrap();

    (*min_distance as i64).into()
}

fn transform_input_for_part_1(input: &str) -> (Position, Position, Heightmap) {
    let mut start_position = None;
    let mut end_position = None;

    'outer: for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if start_position.is_some() && end_position.is_some() {
                break 'outer;
            }
            if c == 'S' {
                start_position = Some(Position::new(column as i32, row as i32))
            } else if c == 'E' {
                end_position = Some(Position::new(column as i32, row as i32))
            }
        }
    }

    let start_position = start_position.expect("input should have a start position");
    let end_position = end_position.expect("input should have an end position");

    (
        start_position,
        end_position,
        Heightmap::from_str(input).expect("input should be a valid heightmap"),
    )
}

fn transform_input_for_part_2(input: &str) -> (Vec<Position>, Position, Heightmap) {
    let mut start_positions = vec![];
    let mut end_position = None;

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == 'S' || c == 'a' {
                start_positions.push(Position::new(column as i32, row as i32))
            } else if c == 'E' {
                end_position = Some(Position::new(column as i32, row as i32))
            }
        }
    }

    let end_position = end_position.expect("input should have an end position");

    (
        start_positions,
        end_position,
        Heightmap::from_str(input).expect("input should be a valid heightmap"),
    )
}

test_solvers!(31, 29);
