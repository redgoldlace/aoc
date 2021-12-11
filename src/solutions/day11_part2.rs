aoc!(day = 11, part = 2);

use super::{day11::tick, day4::Grid};

#[transform]
fn transform(input: _) -> Grid<usize> {
    <day!(11)>::transform(input)
}

#[solve]
fn solve(mut input: _) -> usize {
    let mut step = 0;

    loop {
        tick(&mut input);

        step += 1;

        if input.iter().copied().all(|energy_level| energy_level == 0) {
            break step;
        }
    }
}
