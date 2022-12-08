use std::collections::HashSet;

use crate::{shared::PuzzleResult, test_solvers};

pub fn part1(input: &str) -> PuzzleResult {
    let tree_grid = transform_input(input);

    let v_l = visible_from_left(&tree_grid);
    let v_t = visible_from_top(&tree_grid);
    let v_r = visible_from_right(&tree_grid);
    let v_b = visible_from_bottom(&tree_grid);

    let mut unique = HashSet::new();
    unique.extend(v_l);
    unique.extend(v_t);
    unique.extend(v_r);
    unique.extend(v_b);

    let result = unique.len() as i64;

    result.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let tree_grid = transform_input(input);

    let (rows, cols) = grid_dimensions(&tree_grid);

    let row_indices = 0..rows;
    let col_indices = 0..cols;
    let indices = row_indices.flat_map(|row| col_indices.clone().map(move |col| (row, col)));

    let max_scenic_score = indices
        .map(|pov| scenic_score(&tree_grid, pov))
        .max()
        .expect("max scenic score should exist") as i64;

    max_scenic_score.into()
}

fn scenic_score(tree_grid: &Vec<Vec<u8>>, pov: (usize, usize)) -> u32 {
    let (rows, cols) = grid_dimensions(tree_grid);
    let (pov_row, pov_col) = pov;
    let pov_height = tree_grid[pov_row][pov_col];

    let mut viewing_distances: [u8; 4] = [0, 0, 0, 0];

    // from left
    for col in (0..pov_col).rev() {
        viewing_distances[0] += 1;
        let tree_height = tree_grid[pov_row][col];
        if tree_height >= pov_height {
            break;
        }
    }

    // from top
    for row in (0..pov_row).rev() {
        viewing_distances[1] += 1;
        let tree_height = tree_grid[row][pov_col];
        if tree_height >= pov_height {
            break;
        }
    }

    // from right
    for col in (pov_col + 1)..cols {
        viewing_distances[2] += 1;
        let tree_height = tree_grid[pov_row][col];
        if tree_height >= pov_height {
            break;
        }
    }

    // from bottom
    for row in (pov_row + 1)..rows {
        viewing_distances[3] += 1;
        let tree_height = tree_grid[row][pov_col];
        if tree_height >= pov_height {
            break;
        }
    }

    viewing_distances
        .iter()
        .fold(1, |score, &distance| score * (distance as u32))
}

fn grid_dimensions(tree_grid: &Vec<Vec<u8>>) -> (usize, usize) {
    (tree_grid.len(), tree_grid[0].len())
}

fn visible_from_left(tree_grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let (rows, cols) = grid_dimensions(tree_grid);

    let mut visible = vec![];

    for row in 0..rows {
        let mut highest_tree_height = None;
        for col in 0..cols {
            let current_tree_height = tree_grid[row][col];
            if highest_tree_height.is_none() || current_tree_height > highest_tree_height.unwrap() {
                visible.push((row, col));
                highest_tree_height = Some(current_tree_height);
            }
        }
    }

    visible
}

fn visible_from_top(tree_grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let (rows, cols) = grid_dimensions(tree_grid);

    let mut visible = vec![];

    for col in 0..cols {
        let mut highest_tree_height = None;
        for row in 0..rows {
            let current_tree_height = tree_grid[row][col];
            if highest_tree_height.is_none() || current_tree_height > highest_tree_height.unwrap() {
                visible.push((row, col));
                highest_tree_height = Some(current_tree_height);
            }
        }
    }

    visible
}

fn visible_from_right(tree_grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let (rows, cols) = grid_dimensions(tree_grid);

    let mut visible = vec![];

    for row in 0..rows {
        let mut highest_tree_height = None;
        for col in (0..cols).rev() {
            let current_tree_height = tree_grid[row][col];
            if highest_tree_height.is_none() || current_tree_height > highest_tree_height.unwrap() {
                visible.push((row, col));
                highest_tree_height = Some(current_tree_height);
            }
        }
    }

    visible
}

fn visible_from_bottom(tree_grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let (rows, cols) = grid_dimensions(tree_grid);

    let mut visible = vec![];

    for col in 0..cols {
        let mut highest_tree_height = None;
        for row in (0..rows).rev() {
            let current_tree_height = tree_grid[row][col];
            if highest_tree_height.is_none() || current_tree_height > highest_tree_height.unwrap() {
                visible.push((row, col));
                highest_tree_height = Some(current_tree_height);
            }
        }
    }

    visible
}

fn transform_input(input: &str) -> Vec<Vec<u8>> {
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
