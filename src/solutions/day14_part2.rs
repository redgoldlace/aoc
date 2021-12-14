aoc!(day = 14, part = 2);

use std::collections::HashMap;
use super::day14::{collapse, step, element_delta, Pair};

#[transform]
fn transform(input: _) -> (Vec<char>, HashMap<Pair, char>) {
    <day!(14)>::transform(input)
}


#[solve]
fn solve(input: _) -> usize {
    let (template, replacements) = input;
    let mut frequency = collapse(template);

    for _ in 0..40 {
        frequency = step(frequency, &replacements);
    }

    element_delta(&frequency)
}
