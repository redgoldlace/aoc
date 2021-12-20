aoc!(day = 19, part = 2);

use super::day19::{align, sub, sum, Scanner, Triple};
use itertools::Itertools;

#[transform]
fn transform(input: &str) -> Vec<Scanner> {
    <day!(19)>::transform(input)
}

#[solve]
fn solve(mut input: _) -> usize {
    align(&mut input);

    input
        .iter()
        .permutations(2)
        .map(|window| (window[0], window[1]))
        .fold(0, |highest, (a, b)| {
            // Yuck. Realistically all of these should be trait methods but.. no.
            highest.max(sum(abs(sub(a.position.unwrap(), b.position.unwrap()))) as usize)
        })
}

pub fn abs((a, b, c): Triple) -> Triple {
    (a.abs(), b.abs(), c.abs())
}
