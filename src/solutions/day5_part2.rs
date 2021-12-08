aoc!(day = 5, part = 2);

use super::{
    day4::Coordinate,
    day5::{adjacent, line},
};
use std::collections::HashMap;

#[transform]
fn transform(input: _) -> Vec<(Coordinate, Coordinate)> {
    <day!(5)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    let mut map = HashMap::<Coordinate, usize>::new();

    for (start, end) in input.iter().copied() {
        // This is a boolean logic hack that works because We're Lucky™ (lines can never be adjacent *and* diagonal)
        if adjacent(start, end) == diagonal(start, end) {
            continue;
        }

        for point in line(start, end) {
            *map.entry(point).or_default() += 1;
        }
    }

    map.values().copied().filter(|&count| count >= 2).count()
}

pub fn diagonal((x1, y1): Coordinate, (x2, y2): Coordinate) -> bool {
    (x1 as isize - x2 as isize).abs() == (y1 as isize - y2 as isize).abs()
}
