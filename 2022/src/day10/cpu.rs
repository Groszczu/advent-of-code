use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    pub fn cycles(&self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }

    pub fn update_registers(&self, prev_value: i32) -> i32 {
        match self {
            Self::Noop => prev_value,
            Self::AddX(added_value) => prev_value + added_value,
        }
    }
}

#[derive(Debug)]
pub enum ParseInstructionError {
    InvalidInstructionError,
    InvalidAddXValueError(ParseIntError),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();

        match parts.as_slice() {
            ["noop"] => Ok(Self::Noop),
            ["addx", value] => {
                let value = value
                    .parse()
                    .map_err(ParseInstructionError::InvalidAddXValueError)?;

                Ok(Self::AddX(value))
            }
            _ => Err(ParseInstructionError::InvalidInstructionError),
        }
    }
}

pub struct CPUInstruction {
    instruction: Instruction,
    running_for: u32,
}

impl CPUInstruction {
    pub fn new(instruction: Instruction) -> Self {
        Self {
            instruction,
            running_for: 0,
        }
    }

    pub fn run_cycle(&mut self) -> () {
        self.running_for += 1
    }

    pub fn running_for(&self) -> u32 {
        self.running_for
    }

    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }
}

pub struct CPU {
    register_x: i32,
    cycle: u32,
    current_instruction: Option<CPUInstruction>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register_x: 1,
            cycle: 0,
            current_instruction: None,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.current_instruction.is_some()
    }

    pub fn run_cycle(&mut self) -> (u32, i32) {
        self.cycle += 1;
        let saved_register_x = self.register_x;

        let current_instruction = self
            .current_instruction
            .as_mut()
            .expect("CPU should have an instruction to process");

        current_instruction.run_cycle();

        if current_instruction.running_for() == current_instruction.instruction().cycles() {
            self.register_x = current_instruction
                .instruction()
                .update_registers(self.register_x);
            self.current_instruction = None;
        }

        (self.cycle, saved_register_x)
    }

    pub fn exec(&mut self, instruction: Instruction) -> () {
        self.current_instruction = Some(CPUInstruction::new(instruction));
    }
}
