use std::str::FromStr;

#[derive(Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    pub fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseGameResultError;

impl FromStr for GameResult {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(ParseGameResultError),
        }
    }

    type Err = ParseGameResultError;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn wins_with(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn draws_with(&self) -> Self {
        *self
    }

    pub fn loses_with(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn get_result(&self, other: &Self) -> GameResult {
        if self == other {
            return GameResult::Draw;
        }
        if self.wins_with() == *other {
            return GameResult::Win;
        }
        return GameResult::Lose;
    }
}

#[derive(Debug, Clone)]
pub struct ParseMoveError;

impl FromStr for Move {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseMoveError),
        }
    }

    type Err = ParseMoveError;
}
