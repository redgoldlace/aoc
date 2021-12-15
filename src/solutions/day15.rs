aoc!(day = 15, part = 1);

use super::day4::{Coordinate, Grid};
use rustc_hash::FxHashMap;
use std::{cmp::Ordering, collections::BinaryHeap};

#[transform]
fn transform(input: _) -> Grid<usize> {
    let mut buffer = Vec::with_capacity(100 * 100);

    buffer.extend(input.lines().flat_map(|line| {
        line.trim()
            .chars()
            .map(|number| number.to_digit(10).unwrap() as usize)
    }));

    Grid::from_raw_parts(100, 100, buffer.into_boxed_slice())
}

#[solve]
fn solve(input: _) -> usize {
    djikstra(&input, (0, 0), (99, 99)).unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(usize, Coordinate);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let State(cost, position) = self;
        let State(other_cost, other_position) = other;

        other_cost
            .cmp(cost)
            .then_with(|| position.cmp(other_position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn djikstra(grid: &Grid<usize>, start: Coordinate, end: Coordinate) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut distance: FxHashMap<_, _> = grid
        .coordinates()
        .map(|coordinate| (coordinate, usize::MAX))
        .collect();

    distance.insert(start, 0);
    heap.push(State(0, start));

    while let Some(State(cost, position)) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > distance[&position] {
            continue;
        }

        for (adjacent_position, &adjacent_cost) in grid.adjacent(position) {
            let new_cost = cost + adjacent_cost;

            if new_cost < distance[&adjacent_position] {
                distance.insert(adjacent_position, new_cost);
                heap.push(State(new_cost, adjacent_position));
            }
        }
    }

    None
}
