pub use crate::prelude::*;

impl<'a> Solution<'a> for Day<1, { Part::One }> {
    type Transformed = Vec<usize>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect()
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        input
            .windows(2)
            .map(|window| (window[0] < window[1]) as usize)
            .sum()
    }
}
