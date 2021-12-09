aoc!(day = 9, part = 2);

use std::collections::HashSet;

use super::day4::{Coordinate, Grid};

#[transform]
fn transform(input: _) -> Grid<usize> {
    <day!(9)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    let mut basins = input
        .coordinates()
        .zip(input.iter().copied())
        .filter_map(|(coordinate, n)| {
            input
                .adjacent(coordinate)
                .all(|(_, &adjacent)| adjacent > n)
                .then(|| coordinate)
        })
        .map(|low_point| select(&input, low_point).len())
        .collect::<Vec<_>>();

    basins.sort();
    basins.iter().rev().take(3).product()
}

pub fn select(grid: &Grid<usize>, position: Coordinate) -> HashSet<Coordinate> {
    // This is essentially just a stack-based DFS or similar. Since all basins in the input are wrapped by 9s, our lives
    // are really easy.

    let mut set = HashSet::new();
    let mut stack = vec![position];

    while let Some(position) = stack.pop() {
        if grid[position] == 9 || set.contains(&position) {
            continue;
        }

        set.insert(position);
        stack.extend(grid.adjacent(position).map(|(adjacent, _)| adjacent))
    }

    set
}
