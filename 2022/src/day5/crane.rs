use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Container {
    label: char,
}

impl Container {
    pub fn label(&self) -> char {
        self.label
    }
}

impl From<char> for Container {
    fn from(label: char) -> Self {
        Self { label }
    }
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

impl FromIterator<usize> for MoveInstruction {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let quantity = iter.next().expect("iter should have at least 1 element");
        let from = iter.next().expect("iter should have at least 2 elements");
        let to = iter.next().expect("iter should have at least 3 elements");

        Self { quantity, from, to }
    }
}

pub enum CrateMover {
    V9000,
    V9001,
}

impl CrateMover {
    pub fn order_containers(&self, containers_to_move: Vec<Container>) -> Vec<Container> {
        match self {
            Self::V9000 => containers_to_move.into_iter().rev().collect::<Vec<_>>(),
            Self::V9001 => containers_to_move,
        }
    }
}

#[derive(Debug)]
pub struct Crane {
    containers_stacks: Vec<Vec<Container>>,
}

impl Crane {
    fn pick_containers_up<'a>(&mut self, move_instruction: &MoveInstruction) -> Vec<Container> {
        let from_index = move_instruction.from - 1;
        let from_len = self.containers_stacks[from_index].len();

        self.containers_stacks[from_index].split_off(from_len - move_instruction.quantity)
    }

    fn put_containers_down(
        &mut self,
        move_instruction: &MoveInstruction,
        containers_iter: impl IntoIterator<Item = Container>,
    ) -> () {
        let to_index = move_instruction.to - 1;
        self.containers_stacks[to_index].extend(containers_iter);
    }

    pub fn move_containers(
        &mut self,
        move_instruction: &MoveInstruction,
        crate_mover: CrateMover,
    ) -> () {
        let containers_to_move = self.pick_containers_up(move_instruction);

        let ordered_containers = crate_mover.order_containers(containers_to_move);
        self.put_containers_down(move_instruction, ordered_containers);
    }

    pub fn top_containers(&self) -> impl Iterator<Item = Option<&Container>> {
        self.containers_stacks.iter().map(|stack| stack.last())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseCraneError {
    ExpectedSeparatorError(char),
    ExpectedNextTokenToExistError,
    UnexpectedTokenError(char),
}

impl FromStr for Crane {
    type Err = ParseCraneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.chars().peekable();

        let mut stack_index = 0usize;
        let mut expect_separator = false;
        let mut containers_stacks = Vec::<Vec<Container>>::new();

        while let Some(token) = tokens.next() {
            if containers_stacks.get(stack_index).is_none() {
                containers_stacks.push(Vec::new());
            }
            if expect_separator {
                if token == ' ' {
                    stack_index += 1;
                } else if token == '\n' {
                    stack_index = 0;
                } else {
                    return Err(ParseCraneError::ExpectedSeparatorError(token));
                }
                expect_separator = false;
                continue;
            }
            if token == '[' {
                let label = tokens
                    .peek()
                    .ok_or(ParseCraneError::ExpectedNextTokenToExistError)?;
                let container = Container::from(*label);
                containers_stacks[stack_index].insert(0, container);

                tokens.next();
                tokens.next();
                expect_separator = true;
            } else if token == ' ' {
                if tokens
                    .peek()
                    .ok_or(ParseCraneError::ExpectedNextTokenToExistError)?
                    .is_numeric()
                {
                    break;
                }

                tokens.next();
                tokens.next();
                expect_separator = true;
            } else {
                return Err(ParseCraneError::UnexpectedTokenError(token));
            }
        }

        Ok(Crane { containers_stacks })
    }
}
