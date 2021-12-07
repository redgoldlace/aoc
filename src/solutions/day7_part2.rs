use crate::prelude::*;

impl<'a> Solution<'a> for Day<7, { Part::Two }> {
    type Transformed = Vec<usize>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        Day::<7, { Part::One }>::transform(input)
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        (0_usize..=input.iter().copied().max().unwrap())
            .map(|position| {
                input
                    .iter()
                    .copied()
                    .map(|crab| difference(crab, position))
                    .sum::<usize>()
            })
            .min()
            .unwrap()
    }
}

// Part 2 is very difficult
pub fn difference(a: usize, b: usize) -> usize {
    let difference = super::day7::difference(a, b);

    (difference * (difference + 1)) / 2
}
