use std::{char::from_digit, collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ParseMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(ParseMotionError::InvalidDirectionError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Motion {
    direction: Direction,
    length: u32,
}

impl Motion {
    pub fn new(direction: Direction, length: u32) -> Self {
        Self { direction, length }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn to_position(&self) -> Position {
        let motion_length = self.length() as i32;
        match self.direction {
            Direction::Up => Position::with_coordinates(0, -motion_length),
            Direction::Right => Position::with_coordinates(motion_length, 0),
            Direction::Down => Position::with_coordinates(0, motion_length),
            Direction::Left => Position::with_coordinates(-motion_length, 0),
        }
    }
}

#[derive(Debug)]
pub enum ParseMotionError {
    InvalidPartsError,
    InvalidDirectionError,
    InvalidLengthError,
}

impl FromStr for Motion {
    type Err = ParseMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();

        match parts.as_slice() {
            [direction, length] => {
                let direction = Direction::from_str(direction)?;
                let length = length
                    .parse()
                    .map_err(|_| ParseMotionError::InvalidLengthError)?;

                Ok(Self { direction, length })
            }
            _ => Err(ParseMotionError::InvalidPartsError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self::with_coordinates(0, 0)
    }

    pub fn with_coordinates(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: Self) -> Self {
        Self::with_coordinates(self.x + other.x, self.y + other.y)
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn compare(&self, other: &Self) -> Self {
        let x_comparison = self.x.cmp(&other.x) as i32;
        let y_comparison = self.y.cmp(&other.y) as i32;

        Self::with_coordinates(x_comparison, y_comparison)
    }

    pub fn is_touching(&self, other: &Self) -> bool {
        let x_distance = (self.x - other.x).abs();
        let y_distance = (self.y - other.y).abs();

        x_distance <= 1 && y_distance <= 1
    }
}

pub struct Rope {
    positions: Vec<Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert(Position::new());

        Self {
            positions: vec![Position::new(); length],
            tail_positions,
        }
    }

    pub fn move_head(&mut self, motion: Motion) -> () {
        for _step in 0..motion.length() {
            let motion_step = Motion::new(motion.direction, 1);
            let head = self.positions[0];
            let new_head = head.add(motion_step.to_position());
            self.positions[0] = new_head;

            let mut prev = new_head;
            for tail_index in 1..self.positions.len() {
                let tail = self.positions[tail_index];
                if !prev.is_touching(&tail) {
                    self.move_tail(tail_index);
                }
                prev = self.positions[tail_index];
            }
        }
    }

    fn move_tail(&mut self, tail_index: usize) -> () {
        let head = self.positions[tail_index - 1];
        let tail = self.positions[tail_index];

        let difference_position = head.compare(&tail);

        self.positions[tail_index] = tail.add(difference_position);

        if tail_index == self.positions.len() - 1 {
            self.tail_positions.insert(self.positions[tail_index]);
        }
    }

    pub fn tail_positions(&self) -> &HashSet<Position> {
        &self.tail_positions
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_positions = self.positions.iter().map(|position| position.x());
        let y_positions = self.positions.iter().map(|position| position.y());

        let min_x = x_positions.clone().min().unwrap();
        let max_x = x_positions.max().unwrap();

        let min_y = y_positions.clone().min().unwrap();
        let max_y = y_positions.max().unwrap();

        for y in (min_y - 2)..=(max_y + 2) {
            for x in (min_x - 2)..=(max_x + 2) {
                let rope_position = self
                    .positions
                    .iter()
                    .enumerate()
                    .find(|(_, &position)| position == Position::with_coordinates(x, y));

                match rope_position {
                    Some((index, _)) => {
                        let label = if index == 0 {
                            'H'
                        } else {
                            from_digit(index as u32, 10).unwrap()
                        };
                        write!(f, "{}", label)?
                    }
                    None => write!(f, ".")?,
                };
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
