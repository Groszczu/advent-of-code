use std::{collections::HashSet, str::FromStr};

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

    pub fn is_touching(&self, other: &Self) -> bool {
        let x_distance = (self.x - other.x).abs();
        let y_distance = (self.y - other.y).abs();

        x_distance <= 1 && y_distance <= 1
    }
}

pub struct Rope {
    head: Position,
    tail: Position,
    tail_positions: HashSet<Position>,
}

impl Rope {
    pub fn new() -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert(Position::new());

        Self {
            head: Position::new(),
            tail: Position::new(),
            tail_positions,
        }
    }

    pub fn move_head(&mut self, motion: Motion) -> () {
        for _step in 0..motion.length() {
            let motion_step = Motion::new(motion.direction, 1);
            let new_head = self.head.add(motion_step.to_position());
            self.head = new_head;
            if !self.head.is_touching(&self.tail) {
                self.move_tail();
            }
        }
    }

    fn move_tail(&mut self) -> () {
        let head = self.head;
        let tail = self.tail;

        if head.y() == tail.y() {
            self.move_tail_horizontally();
        } else if head.x() == tail.x() {
            self.move_tail_vertically();
        } else {
            self.move_tail_diagonally();
        }

        self.tail_positions.insert(self.tail);
    }

    fn move_tail_horizontally(&mut self) -> () {
        let motion_position = if self.head.x() < self.tail.x() {
            Position::with_coordinates(-1, 0)
        } else {
            Position::with_coordinates(1, 0)
        };

        self.tail = self.tail.add(motion_position);
    }

    fn move_tail_vertically(&mut self) -> () {
        let motion_position = if self.head.y() < self.tail.y() {
            Position::with_coordinates(0, -1)
        } else {
            Position::with_coordinates(0, 1)
        };

        self.tail = self.tail.add(motion_position);
    }

    fn move_tail_diagonally(&mut self) -> () {
        let head = self.head;
        let tail = self.tail;

        let motion_position = if head.y() < tail.y() && head.x() < tail.x() {
            Position::with_coordinates(-1, -1)
        } else if head.y() < tail.y() && head.x() > tail.x() {
            Position::with_coordinates(1, -1)
        } else if head.y() > tail.y() && head.x() < tail.x() {
            Position::with_coordinates(-1, 1)
        } else if head.y() > tail.y() && head.x() > tail.x() {
            Position::with_coordinates(1, 1)
        } else {
            panic!("unexpected head and tail positions")
        };

        self.tail = self.tail.add(motion_position);
    }

    pub fn tail_positions(&self) -> &HashSet<Position> {
        &self.tail_positions
    }
}
