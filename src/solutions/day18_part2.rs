aoc!(day = 18, part = 2);

use super::day18::{add, magnitude, Pair};
use itertools::Itertools;

#[transform]
fn transform(input: _) -> Vec<Vec<Pair>> {
    <day!(18)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    input
        .iter()
        .permutations(2)
        .map(|permutation| magnitude(add(permutation[0], permutation[1])))
        .max()
        .unwrap()
}
