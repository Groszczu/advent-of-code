use std::fs;

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

                assert_eq!(result, $part1_expected_result);
            }

            #[test]
            fn part2_returns_correct_result_for_test_input() {
                let input = get_test_input();
                let result = part2(&input);

                assert_eq!(result, $part2_expected_result);
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
        const $name: [[fn(&str) -> i64; 2]; $crate::count_tts!($($day)*)] = [
            $([$day::part1, $day::part2]),*
        ];
    };
}
