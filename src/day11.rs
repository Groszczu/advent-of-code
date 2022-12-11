use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::{shared::PuzzleResult, test_solvers};

#[derive(Debug)]
enum Operand {
    Number(u64),
    Old,
}

impl FromStr for Operand {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            num => {
                let num = num
                    .parse::<u64>()
                    .map_err(|_| ParseMonkeyError::InvalidOperationError)?;
                Ok(Self::Number(num))
            }
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: fn(u64, u64) -> u64,
    operand: Operand,
}

impl Operation {
    fn exec(&self, input: u64) -> u64 {
        match self.operand {
            Operand::Number(num) => (self.operator)(input, num),
            Operand::Old => (self.operator)(input, input),
        }
    }

    fn add_operator(a: u64, b: u64) -> u64 {
        a + b
    }

    fn multiply_operator(a: u64, b: u64) -> u64 {
        a * b
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Operation {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = s
            .strip_prefix("Operation: new = ")
            .ok_or(ParseMonkeyError::InvalidOperationError)?;

        let parts = operation.split(' ').collect::<Vec<_>>();

        match parts.as_slice() {
            ["old", "+", operand] => Ok(Self {
                operand: Operand::from_str(operand)?,
                operator: Self::add_operator,
            }),
            ["old", "*", operand] => Ok(Self {
                operand: Operand::from_str(operand)?,
                operator: Self::multiply_operator,
            }),
            _ => Err(ParseMonkeyError::InvalidOperationError),
        }
    }
}

impl Monkey {
    fn new(
        id: usize,
        items: Vec<u64>,
        operation: Operation,
        test: u64,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            id,
            items,
            operation,
            test,
            if_true,
            if_false,
        }
    }

    fn catch_item(&mut self, item: u64) -> () {
        self.items.insert(0, item);
    }

    fn inspect_item(&mut self) -> (usize, u64) {
        let item_to_inspect = self
            .items
            .pop()
            .expect("monkey should have item to inspect");

        let inspected_item = self.operation.exec(item_to_inspect);

        let inspected_item = (inspected_item as f32) / 3.0;

        let inspected_item = inspected_item.floor() as u64;

        let throw_to_monkey = if inspected_item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        };

        (throw_to_monkey, inspected_item)
    }

    fn has_items(&self) -> bool {
        self.items.len() != 0
    }
}

#[derive(Debug)]
enum ParseMonkeyError {
    MissingLineError,
    InvalidIdError,
    InvalidStartingItemsError,
    InvalidOperationError,
    InvalidTestError,
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id_line = lines
            .next()
            .ok_or(ParseMonkeyError::MissingLineError)?
            .trim();

        let id: usize = id_line
            .strip_prefix("Monkey ")
            .and_then(|id| id.strip_suffix(":"))
            .and_then(|id| id.parse::<usize>().ok())
            .ok_or(ParseMonkeyError::InvalidIdError)?;

        let starting_items = lines
            .next()
            .ok_or(ParseMonkeyError::MissingLineError)?
            .trim();

        let starting_items = starting_items
            .strip_prefix("Starting items: ")
            .ok_or(ParseMonkeyError::InvalidStartingItemsError)?;

        let starting_items = starting_items
            .split(", ")
            .filter_map(|item| item.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let operation = lines
            .next()
            .ok_or(ParseMonkeyError::MissingLineError)?
            .trim();
        let operation = Operation::from_str(operation)?;

        let test = lines
            .next()
            .ok_or(ParseMonkeyError::MissingLineError)?
            .trim();

        let test = test
            .strip_prefix("Test: divisible by ")
            .and_then(|test| test.parse::<u64>().ok())
            .ok_or(ParseMonkeyError::InvalidTestError)?;

        let if_true = lines
            .next()
            .ok_or(ParseMonkeyError::MissingLineError)?
            .trim();

        let if_true = if_true
            .strip_prefix("If true: throw to monkey ")
            .and_then(|test| test.parse::<usize>().ok())
            .ok_or(ParseMonkeyError::InvalidTestError)?;

        let if_false = lines
            .next()
            .ok_or(ParseMonkeyError::MissingLineError)?
            .trim();

        let if_false = if_false
            .strip_prefix("If false: throw to monkey ")
            .and_then(|test| test.parse::<usize>().ok())
            .ok_or(ParseMonkeyError::InvalidTestError)?;

        let monkey = Monkey::new(id, starting_items, operation, test, if_true, if_false);

        Ok(monkey)
    }
}

pub fn part1(input: &str) -> PuzzleResult {
    let mut monkeys = transform_input(input);
    let mut inspections: HashMap<usize, u64> = HashMap::new();

    for round in 0..20 {
        for mut monkey_id in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_id];
            let mut throws = vec![];
            while monkey.has_items() {
                throws.push(monkey.inspect_item());
                *inspections.entry(monkey_id).or_insert(0) += 1;
            }

            for throw in throws {
                let catcher = &mut monkeys[throw.0];
                catcher.catch_item(throw.1);
            }
        }
    }

    println!("{:?}", inspections);

    let max_two = inspections.into_iter().fold([0, 0], |mut max, (_, value)| {
        let (lowest_index, &lowest_value) = max
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        if value > lowest_value {
            max[lowest_index] = value;
        }
        max
    });

    let result = (max_two[0] * max_two[1]) as i64;

    result.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut monkeys = transform_input(input);
    let mut inspections: HashMap<usize, u64> = HashMap::new();

    for round in 0..10000 {
        for mut monkey_id in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_id];
            let mut throws = vec![];
            while monkey.has_items() {
                throws.push(monkey.inspect_item());
                *inspections.entry(monkey_id).or_insert(0) += 1;
            }

            for throw in throws {
                let catcher = &mut monkeys[throw.0];
                catcher.catch_item(throw.1);
            }
        }
    }

    println!("{:?}", inspections);

    let max_two = inspections.into_iter().fold([0, 0], |mut max, (_, value)| {
        let (lowest_index, &lowest_value) = max
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        if value > lowest_value {
            max[lowest_index] = value;
        }
        max
    });

    let result = (max_two[0] * max_two[1]) as i64;

    result.into()
}

fn transform_input(input: &str) -> Vec<Monkey> {
    input
        .split_terminator("\n\n")
        .filter_map(|input| Monkey::from_str(input).ok())
        .collect::<Vec<_>>()
}

test_solvers!(0, 0);
