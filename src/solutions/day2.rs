aoc!(day = 2, part = 1);

use std::{convert::Infallible, iter::Sum, ops::Add, str::FromStr};

#[transform]
fn transform(input: _) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    let final_position = input
        .into_iter()
        .map(Into::<Movement>::into)
        .sum::<Movement>();

    (final_position.horizontal * final_position.depth) as usize
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut split = value.split_whitespace();
        let direction = split.next().unwrap();
        let amount = split.next().unwrap().parse::<usize>().unwrap();

        match direction {
            "forward" => Ok(Instruction::Forward(amount)),
            "up" => Ok(Instruction::Up(amount)),
            "down" => Ok(Instruction::Down(amount)),
            _ => unreachable!(),
        }
    }
}

pub enum Instruction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, Default)]
pub struct Movement {
    horizontal: isize,
    depth: isize,
}

impl Movement {
    pub fn new(horizontal: isize, depth: isize) -> Self {
        Self { horizontal, depth }
    }
}

impl From<Instruction> for Movement {
    fn from(instruction: Instruction) -> Self {
        match instruction {
            Instruction::Forward(distance) => Movement::new(distance as isize, 0),
            Instruction::Up(depth) => Movement::new(0, -(depth as isize)),
            Instruction::Down(depth) => Movement::new(0, depth as isize),
        }
    }
}

impl Add for Movement {
    type Output = Self;

    fn add(self, Movement { horizontal, depth }: Self) -> Self::Output {
        Movement::new(self.horizontal + horizontal, self.depth + depth)
    }
}

impl Sum for Movement {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Movement::default(), |a, b| a + b)
    }
}
