use std::collections::HashMap;
use crate::prelude::*;

impl<'a> Solution<'a> for Day<6, { Part::One }> {
    type Transformed = Vec<usize>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        let mut buffer = input
            .split(',')
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        buffer.sort();

        let occurrences = buffer
            .group_by(|&a, &b| a == b)
            .map(|group| (group[0], group.len()))
            .collect::<HashMap<_, _>>();

        println!("{:?}", occurrences);

        (0..9)
            .map(|n| occurrences.get(&n).copied().unwrap_or_default())
            .collect::<Vec<_>>()
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        let mut counts = input;

        for _ in 0..80 {
            tick(&mut counts);
        }

        counts.iter().copied().sum::<usize>()
    }
}

pub fn tick(counts: &mut Vec<usize>) {
    let zeroes = counts[0];
    counts[0..=6].rotate_left(1);
    counts[6] += counts[7];
    counts.swap(7, 8);
    counts[8] = zeroes;
}
