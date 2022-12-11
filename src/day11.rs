use std::str::FromStr;

use crate::{shared::PuzzleResult, test_solvers};

use monkey::Monkey;

mod monkey;

pub fn part1(input: &str) -> PuzzleResult {
    let mut monkeys = transform_input(input);

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_id];
            let mut throws = vec![];
            while monkey.has_items() {
                let item = monkey.inspect_item();
                let item = monkey.lower_worry_level(item);
                let catcher = monkey.get_catcher(item);
                let throw = (catcher, item);
                throws.push(throw);
            }

            for (to, item) in throws {
                let catcher = &mut monkeys[to];
                catcher.catch_item(item);
            }
        }
    }

    monkey_business_level(&monkeys)
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut monkeys = transform_input(input);
    let modulo = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test());

    for _round in 0..10000 {
        for monkey_id in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_id];
            let mut throws = vec![];
            while monkey.has_items() {
                let item = monkey.inspect_item();
                let catcher = monkey.get_catcher(item);
                let throw = (catcher, item);
                throws.push(throw);
            }

            for (to, item) in throws {
                let catcher = &mut monkeys[to];
                catcher.catch_item(item % modulo);
            }
        }
    }

    monkey_business_level(&monkeys)
}

fn monkey_business_level(monkeys: &Vec<Monkey>) -> PuzzleResult {
    let mut sorted_inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections())
        .collect::<Vec<_>>();

    sorted_inspections.sort_by(|a, b| b.cmp(a));

    let result = (sorted_inspections[0] * sorted_inspections[1]) as i64;

    result.into()
}

fn transform_input(input: &str) -> Vec<Monkey> {
    input
        .split_terminator("\n\n")
        .filter_map(|input| Monkey::from_str(input).ok())
        .collect::<Vec<_>>()
}

test_solvers!(10605, 2713310158);
