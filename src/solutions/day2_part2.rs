use super::day2::Instruction;
use crate::prelude::*;
use std::{iter::Sum, ops::Add};

impl<'a> Solution<'a> for Day<2, { Part::Two }> {
    type Transformed = Vec<Instruction>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        input
            .lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect()
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        let final_position = input
            .into_iter()
            .fold(Movement::default(), |position, instruction| {
                position.step_by(instruction)
            });

        (final_position.horizontal * final_position.depth) as usize
    }
}

#[derive(Debug, Default, Clone)]
pub struct Movement {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Movement {
    pub fn new(horizontal: isize, depth: isize, aim: isize) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }

    pub fn step(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(distance) => {
                self.horizontal += distance as isize;
                self.depth += self.aim * distance as isize;
            }
            Instruction::Up(delta) => self.aim -= delta as isize,
            Instruction::Down(delta) => self.aim += delta as isize,
        };
    }

    pub fn step_by(&self, instruction: Instruction) -> Self {
        let mut new = self.clone();
        new.step(instruction);
        new
    }
}

impl Add for Movement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Movement::new(
            self.horizontal + rhs.horizontal,
            self.depth + rhs.depth,
            self.aim + rhs.aim,
        )
    }
}

impl Sum for Movement {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Movement::default(), |a, b| a + b)
    }
}
