use std::str::FromStr;

pub struct Monkey {
    _id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

impl Monkey {
    pub fn test(&self) -> u64 {
        self.test
    }

    pub fn inspections(&self) -> u64 {
        self.inspections
    }

    pub fn has_items(&self) -> bool {
        self.items.len() != 0
    }

    pub fn catch_item(&mut self, item: u64) -> () {
        self.items.insert(0, item);
    }

    pub fn inspect_item(&mut self) -> u64 {
        let item = self.start_inspection();

        item
    }

    pub fn lower_worry_level(&self, item: u64) -> u64 {
        let item = (item as f32) / 3.0;

        item.floor() as u64
    }

    pub fn get_catcher(&mut self, item: u64) -> usize {
        let throw_to_monkey = self.test_item(item);

        throw_to_monkey
    }

    fn start_inspection(&mut self) -> u64 {
        self.inspections += 1;

        let item = self.pop_item();

        self.operation.exec(item)
    }

    fn pop_item(&mut self) -> u64 {
        self.items
            .pop()
            .expect("monkey should have item to inspect")
    }

    fn test_item(&self, item: u64) -> usize {
        if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
pub enum ParseMonkeyError {
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

        Ok(Monkey {
            _id: id,
            items: starting_items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0,
        })
    }
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
    pub operator: fn(u64, u64) -> u64,
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
