aoc!(day = 11, part = 1);

use super::day4::{Coordinate, Grid};
use std::{collections::HashSet};

#[transform]
fn transform(input: _) -> Grid<usize> {
    let buffer = input
        .lines()
        .flat_map(|line| {
            line.trim()
                .chars()
                .map(|letter| letter.to_digit(10).unwrap() as usize)
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Grid::from_raw_parts(10, 10, buffer)
}

#[solve]
fn solve(mut input: _) -> usize {
    (0..100).map(|_| tick(&mut input)).sum()
}

pub fn tick(grid: &mut Grid<usize>) -> usize {
    let mut flashed = HashSet::<Coordinate>::new();
    let mut to_flash = Vec::new();

    for (position, energy_level) in grid.coordinates().zip(grid.iter_mut()) {
        *energy_level += 1;

        if *energy_level > 9 {
            to_flash.push(position);
        }
    }

    while let Some(position) = to_flash.pop() {
        if flashed.contains(&position) {
            continue;
        }

        flashed.insert(position);

        for adjacent_position in grid.diagonally_adjacent(position) {
            grid[adjacent_position] += 1;

            if grid[adjacent_position] > 9 {
                to_flash.push(adjacent_position);
            }
        }
    }

    for flashed_position in flashed.iter().copied() {
        grid[flashed_position] = 0;
    }

    flashed.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum Horizontal {
    Left = -1,
    Middle = 0,
    Right = 1,
}

impl Horizontal {
    pub fn offset(self) -> isize {
        self as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum Vertical {
    Top = -1,
    Middle = 0,
    Bottom = 1,
}

impl Vertical {
    pub fn offset(self) -> isize {
        self as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(Horizontal, Vertical);

impl Position {
    pub const ALL: [Position; 8] = [
        Position(Horizontal::Left, Vertical::Top),
        Position(Horizontal::Middle, Vertical::Top),
        Position(Horizontal::Right, Vertical::Top),
        Position(Horizontal::Left, Vertical::Middle),
        Position(Horizontal::Right, Vertical::Middle),
        Position(Horizontal::Left, Vertical::Bottom),
        Position(Horizontal::Middle, Vertical::Bottom),
        Position(Horizontal::Right, Vertical::Bottom),
    ];

    pub fn offset(self) -> Coordinate<isize> {
        (self.0.offset(), self.1.offset())
    }
}

// Yes this is blatantly copied from a previous round

impl<T> Grid<T> {
    pub fn diagonally_adjacent(&self, (x, y): Coordinate) -> impl Iterator<Item = Coordinate> {
        let (width, height) = (self.width() as isize, self.height() as isize);

        Position::ALL.into_iter().filter_map(move |position| {
            let (offset_x, offset_y) = position.offset();
            let (computed_x, computed_y) = (x as isize + offset_x, y as isize + offset_y);

            // This song and dance is mostly to avoid overflows from `0 - 1` and similar.
            let x_in_bounds = (0..width).contains(&computed_x);
            let y_in_bounds = (0..height).contains(&computed_y);

            (x_in_bounds && y_in_bounds).then(|| (computed_x as usize, computed_y as usize))
        })
    }
}
