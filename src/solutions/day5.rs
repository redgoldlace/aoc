use super::day4::Coordinate;
use crate::prelude::*;
use lerp::Lerp;
use std::collections::HashMap;

impl<'a> Solution<'a> for Day<5, { Part::One }> {
    type Transformed = Vec<(Coordinate, Coordinate)>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        input
            .lines()
            .map(|line| {
                let mut split = line.split("->").map(|pair| parse_point(pair.trim()));

                (split.next().unwrap(), split.next().unwrap())
            })
            .collect()
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        let mut map = HashMap::<Coordinate, usize>::new();

        for (start, end) in input.iter().copied() {
            if !adjacent(start, end) {
                continue;
            }

            for point in line(start, end) {
                *map.entry(point).or_default() += 1;
            }
        }

        map.values().copied().filter(|&count| count >= 2).count()
    }
}

pub fn parse_point(line: &str) -> Coordinate {
    let mut split = line
        .split(',')
        .map(|number| number.parse::<usize>().unwrap());

    (split.next().unwrap(), split.next().unwrap())
}

pub fn adjacent((x1, y1): Coordinate, (x2, y2): Coordinate) -> bool {
    x1 == x2 || y1 == y2
}

pub fn line(start: Coordinate, stop: Coordinate) -> impl Iterator<Item = Coordinate> {
    let distance = diagonal_distance(start, stop);

    range(0, distance).map(move |step| {
        let time = {
            if step == 0 {
                0.0
            } else {
                step as f64 / distance as f64
            }
        };

        lerp_coordinate(start, stop, time)
    })
}

// Rust's ranges suck and will sputter and die if reverse ranges are involved
pub fn range(start: usize, stop: usize) -> Box<dyn Iterator<Item = usize>> {
    if start > stop {
        Box::new((stop..=start).rev())
    } else {
        Box::new(start..=stop)
    }
}

pub fn lerp_coordinate((x1, y1): Coordinate, (x2, y2): Coordinate, time: f64) -> Coordinate {
    (
        (x1 as f64).lerp(x2 as f64, time).round() as usize,
        (y1 as f64).lerp(y2 as f64, time).round() as usize,
    )
}

pub fn diagonal_distance((x1, y1): Coordinate, (x2, y2): Coordinate) -> usize {
    let x = (x1 as isize - x2 as isize).abs() as usize;
    let y = (y1 as isize - y2 as isize).abs() as usize;

    x.max(y)
}
