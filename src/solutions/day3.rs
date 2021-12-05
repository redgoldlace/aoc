use crate::prelude::*;

impl<'a> Solution<'a> for Day<3, { Part::One }> {
    type Transformed = Vec<Vec<bool>>;
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(2).unwrap() != 0)
                    .collect::<Vec<bool>>()
            })
            .collect()
    }

    fn solve(input: Self::Transformed) -> Self::Result {
        let columns = input[0].len();

        let mut gamma = Vec::with_capacity(columns);
        let mut epsilon = Vec::with_capacity(columns);

        for i in 0..columns {
            let ones = column(&input, i).map(Into::<usize>::into).sum::<usize>();
            let zeroes = input.len() - ones;

            gamma.push(ones > zeroes);
            epsilon.push(ones < zeroes);
        }

        bits_to_n(&gamma) * bits_to_n(&epsilon)
    }
}

pub fn column<'a>(slice: &'a [Vec<bool>], index: usize) -> impl Iterator<Item = bool> + 'a {
    slice.iter().map(move |row| row[index])
}

pub fn bits_to_n(slice: &[bool]) -> usize {
    slice.iter().fold(0, |n, &bit| ((n * 2) + bit as usize))
}
