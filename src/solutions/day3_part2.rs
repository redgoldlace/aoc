aoc!(day = 3, part = 2);

use super::day3::{bits_to_n, column};
use std::cmp::Ordering;

#[transform]
fn transform(input: &'a str) -> Vec<Vec<bool>> {
    <day!(3)>::transform(input)
}

// I had an aneurysm while writing this
#[solve]
fn solve(input: Self::Transformed) -> usize {
    let columns = input[0].len();

    let mut generator = input.clone();
    let mut scrubber = input.clone();

    let filter = |items: &mut Vec<Vec<bool>>, index: usize, criteria: Criteria| {
        if items.len() == 1 {
            return;
        }

        let ones = column(&items, index)
            .map(Into::<usize>::into)
            .sum::<usize>();

        let zeroes = items.len() - ones;
        let choices = [ones < zeroes, ones > zeroes];

        let bit = match zeroes.cmp(&ones) {
            Ordering::Equal => matches!(criteria, Criteria::Generator),
            _ => choices[matches!(criteria, Criteria::Generator) as usize],
        };

        items.retain(|row| row[index] == bit)
    };

    for i in 0..columns {
        filter(&mut generator, i, Criteria::Generator);
        filter(&mut scrubber, i, Criteria::Scrubber);
    }

    assert_eq!(generator.len(), 1, "generator should have one item");
    assert_eq!(scrubber.len(), 1, "scrubber should have one item");

    bits_to_n(&generator[0]) * bits_to_n(&scrubber[0])
}

enum Criteria {
    Generator,
    Scrubber,
}
