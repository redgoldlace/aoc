aoc!(day = 20, part = 1);

use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::ops::RangeInclusive;

#[transform]
fn transform(input: _) -> Grid {
    let mut lines = input.lines();
    let algorithm = lines.next().unwrap().chars().map(from_char).collect();
    let image = lines
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(from_char)
                .enumerate()
                .filter_map(move |(x, lit)| lit.then(|| (x as isize, y as isize)))
        })
        .collect();

    Grid::new(image, algorithm)
}

#[solve]
fn solve(input: _) -> usize {
    let mut grid = input;

    for _ in 0..2 {
        grid.step()
    }

    grid.lit()
}

pub type Coordinate = super::day4::Coordinate<isize>;

pub const ADJACENT: [Coordinate; 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Grid {
    lit: FxHashSet<Coordinate>,
    algorithm: Vec<bool>,
    default: bool,
    x_bounds: RangeInclusive<isize>,
    y_bounds: RangeInclusive<isize>,
}

impl Grid {
    pub fn new(lit: FxHashSet<Coordinate>, algorithm: Vec<bool>) -> Self {
        let mut grid = Grid {
            lit,
            algorithm,
            default: false,
            x_bounds: 0..=0,
            y_bounds: 0..=0,
        };

        grid.update_bounds();

        grid
    }

    pub fn lit(&self) -> usize {
        self.lit.len()
    }

    pub fn update_bounds(&mut self) {
        #[inline]
        fn take<'a>(
            iter: impl IntoIterator<Item = &'a Coordinate>,
            f: fn(&Coordinate) -> isize,
        ) -> Option<RangeInclusive<isize>> {
            iter.into_iter()
                .map(f)
                .minmax()
                .into_option()
                .map(|(a, b)| a..=b)
        }

        self.x_bounds = take(&self.lit, |&(x, _)| x).unwrap();
        self.y_bounds = take(&self.lit, |&(_, y)| y).unwrap();
    }

    pub fn kernel(&self, (x, y): Coordinate) -> usize {
        ADJACENT
            .iter()
            .copied()
            .enumerate()
            .fold(0, |n, (index, (x_offset, y_offset))| {
                n | (self.get((x + x_offset, y + y_offset)) as usize) << (8 - index)
            })
    }

    pub fn step(&mut self) {
        let mut new = FxHashSet::default();

        #[inline]
        fn widen(bounds: &RangeInclusive<isize>) -> RangeInclusive<isize> {
            (bounds.start() - 1)..=(bounds.end() + 1)
        }

        let x_bounds = widen(&self.x_bounds);
        let y_bounds = widen(&self.y_bounds);

        let positions = y_bounds
            .clone()
            .flat_map(|y| x_bounds.clone().map(move |x| (x, y)));

        for position in positions {
            if self.algorithm[self.kernel(position)] {
                new.insert(position);
            }
        }

        self.lit = new;

        let new_default = match self.default {
            true => self.algorithm.last(),
            false => self.algorithm.first(),
        };

        self.default = new_default.copied().unwrap();
        self.x_bounds = x_bounds;
        self.y_bounds = y_bounds;
    }

    pub fn get(&self, position @ (x, y): Coordinate) -> bool {
        if self.x_bounds.contains(&x) && self.y_bounds.contains(&y) {
            self.lit.contains(&position)
        } else {
            self.default
        }
    }
}

pub fn from_char(value: char) -> bool {
    match value {
        '#' => true,
        '.' => false,
        _ => unreachable!(),
    }
}
