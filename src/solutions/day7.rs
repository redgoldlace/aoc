use crate::prelude::*;

impl<'a> Solution<'a> for Day<7, { Part::One }> {
    type Transformed = Vec<usize>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        input
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect()
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

pub fn difference(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as _
}
