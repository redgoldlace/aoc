aoc!(day = 9, part = 1);

use super::day4::{Coordinate, Grid};

#[transform]
fn transform(input: _) -> Grid<usize> {
    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();

    let buffer = input
        .lines()
        .flat_map(|line| line.chars())
        .map(|height| height.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Grid::<usize>::from_raw_parts(width, height, buffer)
}

#[solve]
fn solve(input: _) -> usize {
    input
        .coordinates()
        .zip(input.iter().copied())
        .filter_map(|(coordinate, n)| {
            input
                .adjacent(coordinate)
                .all(|(_, &adjacent)| adjacent > n)
                .then(|| n + 1)
        })
        .sum()
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl<T> Grid<T> {
    pub fn adjacent(&self, (x, y): Coordinate) -> impl Iterator<Item = (Coordinate, &T)> {
        Direction::ALL.into_iter().filter_map(move |direction| {
            let (offset_x, offset_y) = direction.offset();
            let (computed_x, computed_y) = (x as isize + offset_x, y as isize + offset_y);

            // This song and dance is mostly to avoid overflows from `0 - 1` and similar.
            let x_in_bounds = (0..self.width() as isize).contains(&computed_x);
            let y_in_bounds = (0..self.height() as isize).contains(&computed_y);

            if !x_in_bounds || !y_in_bounds {
                return None;
            }

            let index = (computed_x as usize, computed_y as usize);
            Some((index, &self[index]))
        })
    }
}
