aoc!(day = 17, part = 1);

use super::day4::Coordinate;
use std::{cmp::Ordering, ops::RangeInclusive};

#[transform]
fn transform(input: _) -> Target {
    let (x, y) = input
        .trim_start_matches("target area:")
        .split_once(',')
        .unwrap();

    // Skip the `x=` and `y=`
    let (x, y) = (&x.trim()[2..], &y.trim()[2..]);

    let (x_min, x_max) = x.split_once("..").unwrap();
    let (y_min, y_max) = y.split_once("..").unwrap();

    Target {
        x: x_min.parse().unwrap()..=x_max.parse().unwrap(),
        y: y_min.parse().unwrap()..=y_max.parse().unwrap(),
    }
}

#[solve]
fn solve(input: _) -> isize {
    let x_range = 0..=*input.x.end();
    let y_range = *input.y.start()..=input.y.start().abs();

    let mut highest = 0;

    for velocity in range_product(x_range, y_range) {
        let Some(height) = simulate(&input, velocity) else {
            continue;
        };

        highest = highest.max(height);
    }

    highest
}

pub fn range_product(
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
) -> impl Iterator<Item = Coordinate<isize>> {
    y_range.flat_map(move |y| x_range.clone().map(move |x| (x, y)))
}

pub struct Target {
    pub(crate) x: RangeInclusive<isize>,
    pub(crate) y: RangeInclusive<isize>,
}

pub struct Probe {
    position: Coordinate<isize>,
    velocity: Coordinate<isize>,
}

impl Probe {
    pub fn new(velocity: Coordinate<isize>) -> Self {
        Self {
            position: (0, 0),
            velocity,
        }
    }

    pub fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        let change = match self.velocity.0.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        self.velocity.0 += change;
        self.velocity.1 -= 1;
    }

    pub fn within(&self, target: &Target) -> bool {
        let (x, y) = self.position;
        target.x.contains(&x) && target.y.contains(&y)
    }
}

pub fn simulate(target: &Target, velocity: Coordinate<isize>) -> Option<isize> {
    let mut probe = Probe::new(velocity);
    let mut highest = 0;

    #[inline]
    fn approaching(target: &Target, probe: &Probe) -> bool {
        let (x, y) = probe.position;

        x < *target.x.end() && y >= *target.y.start()
    }

    while approaching(target, &probe) {
        probe.step();
        highest = probe.position.1.max(highest);

        if probe.within(target) {
            return Some(highest);
        }
    }

    // We suck
    None
}
