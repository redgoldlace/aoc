aoc!(day = 6, part = 2);

use super::day6::tick;

#[transform]
fn transform(input: _) -> Vec<usize> {
    <day!(6)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    let mut counts = input;

    for _ in 0..256 {
        tick(&mut counts);
    }

    counts.iter().copied().sum()
}
