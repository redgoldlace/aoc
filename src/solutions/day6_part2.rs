use super::day6::tick;
use crate::prelude::*;

impl<'a> Solution<'a> for Day<6, { Part::Two }> {
    type Transformed = Vec<usize>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        Day::<6, { Part::One }>::transform(input)
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        let mut counts = input;

        for _ in 0..256 {
            tick(&mut counts);
        }

        counts.iter().copied().sum::<usize>()
    }
}
