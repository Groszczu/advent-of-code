mod cpu;
mod crt;

use std::str::FromStr;

use crate::{shared::PuzzleResult, test_solvers};

use cpu::{Instruction, CPU};
use crt::CRT;

pub fn part1(input: &str) -> PuzzleResult {
    const SIGNAL_STRENGTH_MEASURE_CYCLES: [u32; 6] = [20, 60, 100, 140, 180, 220];

    let instructions = transform_input(input);

    let mut cpu = CPU::new();

    let mut signal_strengths = vec![];

    for instruction in instructions {
        cpu.exec(instruction);
        while cpu.is_busy() {
            let (cycle, register_x) = cpu.run_cycle();
            if SIGNAL_STRENGTH_MEASURE_CYCLES.contains(&cycle) {
                signal_strengths.push((cycle as i32) * register_x);
            }
        }
    }

    let result = signal_strengths.into_iter().sum::<i32>() as i64;

    result.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let instructions = transform_input(input);

    let mut cpu = CPU::new();

    let mut crt = CRT::new();

    for instruction in instructions {
        cpu.exec(instruction);
        while cpu.is_busy() {
            let (cycle, register_x) = cpu.run_cycle();

            crt.draw(cycle, register_x);
        }
    }

    crt.to_string().into()
}

fn transform_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|input| Instruction::from_str(input).ok())
        .collect()
}

test_solvers!(
    13140,
    "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    .to_owned()
);
