aoc!(day = 13, part = 1);

use super::day4::Coordinate;
use std::{collections::HashSet, convert::Infallible, str::FromStr};

#[transform]
fn transform(input: _) -> (Paper, Vec<Fold>) {
    let mut lines = input.lines();

    let positions = lines
        .by_ref()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<HashSet<_>>();

    let instructions = lines
        .by_ref()
        .map(|line| line.trim().parse::<Fold>().unwrap())
        .collect::<Vec<_>>();

    (Paper::from_coordinates(positions), instructions)
}

#[solve]
fn solve(input: _) -> usize {
    let (mut paper, instructions) = input;

    paper.fold(instructions[0]);

    paper.points.iter().count()
}

pub struct Paper {
    pub(crate) points: HashSet<Coordinate>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Paper {
    pub fn from_coordinates(points: HashSet<Coordinate>) -> Self {
        let (widths, heights): (Vec<_>, Vec<_>) = points.iter().copied().unzip();

        // These are 0-indexed values, so we need to add one.
        let width = widths.into_iter().max().unwrap() + 1;
        let height = heights.into_iter().max().unwrap() + 1;

        Self {
            points,
            width,
            height,
        }
    }

    pub fn fold(&mut self, position: Fold) {
        let new_points: Vec<_> = match position {
            Fold::Left(index) => self
                .points
                .drain_filter(|&(x, _)| x > index)
                .map(|(x, y)| ((index * 2) - x, y))
                .collect(),
            Fold::Up(index) => self
                .points
                .drain_filter(|&(_, y)| y > index)
                .map(|(x, y)| (x, (index * 2) - y))
                .collect(),
        };

        match position {
            Fold::Left(x) => self.width = x,
            Fold::Up(y) => self.height = y,
        }

        self.points.extend(new_points);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    Left(usize),
    Up(usize),
}

impl FromStr for Fold {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (instruction, position) = value.split_once('=').unwrap();
        let position = position.parse::<usize>().unwrap();

        match instruction {
            "fold along x" => Ok(Fold::Left(position)),
            "fold along y" => Ok(Fold::Up(position)),
            _ => unreachable!(),
        }
    }
}
