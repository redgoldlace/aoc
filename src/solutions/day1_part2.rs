pub use crate::prelude::*;

impl<'a> Solution<'a> for Day<1, { Part::Two }> {
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
            .windows(3)
            .zip(input.windows(3).skip(1))
            .map(|(a, b)| (a.iter().copied(), b.iter().copied()))
            .map(|(a, b)| (b.sum::<usize>() > a.sum::<usize>()) as usize)
            .sum()
    }
}
