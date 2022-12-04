use crate::test_puzzle;

pub fn part1(input: &str) -> i64 {
    let elf_calories = transform_input(input);

    elf_calories
        .iter()
        .map(|calories| calories.iter().sum::<i64>())
        .max()
        .unwrap_or(0)
}

pub fn part2(input: &str) -> i64 {
    let elf_calories = transform_input(input);

    let top_3_calories = elf_calories
        .iter()
        .map(|calories| calories.iter().sum::<i64>())
        .fold([0, 0, 0], |mut top_3_calories, total_calories| {
            let (lowest_index, &lowest_calories) = top_3_calories
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();

            if total_calories > lowest_calories {
                top_3_calories[lowest_index] = total_calories
            }
            top_3_calories
        });

    top_3_calories.iter().sum::<i64>()
}

fn transform_input(input: &str) -> Vec<Vec<i64>> {
    input
        .split_terminator("\n\n")
        .map(|items_calories| {
            items_calories
                .split_terminator("\n")
                .map(|item_calories| {
                    item_calories
                        .parse::<i64>()
                        .expect("item calorie should be a number")
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

test_puzzle!(1, 24_000, 45_000);
