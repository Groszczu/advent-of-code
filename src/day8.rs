use std::collections::HashSet;

use crate::{shared::PuzzleResult, test_solvers};

type TreeGrid = Vec<Vec<u8>>;

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn traverse_from_edge(&self, tree_grid: &TreeGrid, start_index: usize) -> Vec<Position> {
        let (ys, xs) = grid_dimensions(tree_grid);

        let start_position = match self {
            Self::Top => Position::new(start_index, ys - 1),
            Self::Right => Position::new(0, start_index),
            Self::Bottom => Position::new(start_index, 0),
            Self::Left => Position::new(xs - 1, start_index),
        };

        let (x_modifier, y_modifier) = self.position_modifier();

        let mut highest_tree_height = None::<u8>;

        let mut visible_positions = vec![];

        let mut next_position = Some(start_position);

        while let Some((position, &tree_height)) = next_position.and_then(|position| {
            tree_height(tree_grid, &position).and_then(|height| Some((position, height)))
        }) {
            if highest_tree_height
                .map(|highest| tree_height > highest)
                .unwrap_or(true)
            {
                visible_positions.push(position);
                highest_tree_height = Some(tree_height);
            }

            next_position = position.add(x_modifier, y_modifier);
        }

        visible_positions
    }

    fn traverse_from(&self, tree_grid: &TreeGrid, start_position: Position) -> Vec<Position> {
        let start_position_height = tree_grid[start_position.y][start_position.x];

        let mut visible_positions = vec![];

        let (x_modifier, y_modifier) = self.position_modifier();
        let mut next_position = start_position.add(x_modifier, y_modifier);

        while let Some((position, &tree_height)) = next_position.and_then(|position| {
            tree_height(tree_grid, &position).and_then(|height| Some((position, height)))
        }) {
            visible_positions.push(position);

            if tree_height >= start_position_height {
                break;
            }

            next_position = position.add(x_modifier, y_modifier);
        }

        visible_positions
    }

    fn position_modifier(&self) -> (i32, i32) {
        match self {
            Self::Top => (0, -1),
            Self::Right => (1, 0),
            Self::Bottom => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn add(&self, x: i32, y: i32) -> Option<Position> {
        let new_x = checked_add(self.x, x)?;
        let new_y = checked_add(self.y, y)?;

        Some(Self::new(new_x, new_y))
    }
}

fn checked_add(a: usize, b: i32) -> Option<usize> {
    let result = a as i32 + b;
    if result < 0 {
        None
    } else {
        Some(result as usize)
    }
}

pub fn part1(input: &str) -> PuzzleResult {
    let tree_grid = transform_input(input);

    let (y, x) = grid_dimensions(&tree_grid);

    let horizontal_edges = 0..x;
    let vertical_edges = 0..y;

    let visible_from_top = horizontal_edges
        .clone()
        .flat_map(|x| Direction::Bottom.traverse_from_edge(&tree_grid, x));

    let visible_from_right = vertical_edges
        .clone()
        .flat_map(|y| Direction::Left.traverse_from_edge(&tree_grid, y));

    let visible_from_bottom =
        horizontal_edges.flat_map(|x| Direction::Top.traverse_from_edge(&tree_grid, x));

    let visible_from_left =
        vertical_edges.flat_map(|y| Direction::Right.traverse_from_edge(&tree_grid, y));

    let mut visible_from_outside = HashSet::new();
    visible_from_outside.extend(
        visible_from_top
            .chain(visible_from_right)
            .chain(visible_from_bottom)
            .chain(visible_from_left),
    );

    let result = visible_from_outside.len() as i64;

    result.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let tree_grid = transform_input(input);

    let (ys, xs) = grid_dimensions(&tree_grid);

    let y_indices = 0..ys;
    let x_indices = 0..xs;
    let indices = y_indices.flat_map(|y| x_indices.clone().map(move |x| Position::new(x, y)));

    let max_scenic_score = indices
        .map(|pov| scenic_score(&tree_grid, pov))
        .max()
        .expect("max scenic score should exist") as i64;

    max_scenic_score.into()
}

fn scenic_score(tree_grid: &TreeGrid, start_position: Position) -> u32 {
    [
        Direction::Top,
        Direction::Right,
        Direction::Bottom,
        Direction::Left,
    ]
    .iter()
    .map(|direction| direction.traverse_from(tree_grid, start_position).len())
    .fold(1, |score, distance| score * (distance as u32))
}

fn grid_dimensions(tree_grid: &TreeGrid) -> (usize, usize) {
    (tree_grid.len(), tree_grid[0].len())
}

fn tree_height<'a>(tree_grid: &'a TreeGrid, position: &Position) -> Option<&'a u8> {
    tree_grid
        .get(position.y)
        .and_then(|row| row.get(position.x))
}

fn transform_input(input: &str) -> TreeGrid {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|height| height.parse().ok())
                .collect()
        })
        .collect()
}

test_solvers!(21, 8);
