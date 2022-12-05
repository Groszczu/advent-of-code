use std::{fmt::Display, fs};

#[derive(Debug, PartialEq)]
pub enum PuzzleResult {
    Text(String),
    Num(i64),
}

impl From<i64> for PuzzleResult {
    fn from(value: i64) -> Self {
        Self::Num(value)
    }
}

impl From<String> for PuzzleResult {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl Display for PuzzleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleResult::Num(value) => value.fmt(f),
            PuzzleResult::Text(value) => value.fmt(f),
        }
    }
}

pub fn read_input(day: u8, filename: &str) -> String {
    let full_path = format!("inputs/day{day}/{filename}.txt");
    fs::read_to_string(full_path).expect("input file should exist")
}

#[macro_export]
macro_rules! test_puzzle {
    ($day:expr, $part1_expected_result:expr, $part2_expected_result:expr) => {
        #[cfg(test)]
        mod tests {
            use super::{part1, part2};
            use crate::shared;

            fn get_test_input() -> String {
                shared::read_input($day, "test")
            }

            #[test]
            fn part1_returns_correct_result_for_test_input() {
                let input = get_test_input();
                let result = part1(&input);
                let wrapped_expected_result: crate::shared::PuzzleResult =
                    $part1_expected_result.into();

                assert_eq!(result, wrapped_expected_result);
            }

            #[test]
            fn part2_returns_correct_result_for_test_input() {
                let input = get_test_input();
                let result = part2(&input);
                let wrapped_expected_result: crate::shared::PuzzleResult =
                    $part2_expected_result.into();

                assert_eq!(result, wrapped_expected_result);
            }
        }
    };
}

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_export]
macro_rules! count_tts {
    ($($tts:tt)*) => {0usize $(+ $crate::replace_expr!($tts 1usize))*};
}

#[macro_export]
macro_rules! define_solvers {
    ($name:ident, $($day:ident),*) => {
        const $name: [[fn(&str) -> shared::PuzzleResult; 2]; $crate::count_tts!($($day)*)] = [
            $([$day::part1, $day::part2]),*
        ];
    };
}
