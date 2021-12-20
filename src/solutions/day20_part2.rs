aoc!(day = 20, part = 2);

use super::day20::Grid;

#[transform]
fn transform(input: _) -> Grid {
    <day!(20)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    let mut grid = input;

    for _ in 0..50 {
        grid.step()
    }

    grid.lit()
}
