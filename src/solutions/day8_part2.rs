aoc!(day = 8, part = 2);

use super::day8::{Display, Segment};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{BTreeSet, HashMap};

#[transform]
fn transform(input: _) -> Vec<(Vec<Display>, Vec<Display>)> {
    <day!(8)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    input
        .into_iter()
        .map(|(entries, outputs)| solve_connections(entries, outputs))
        .sum()
}

lazy_static! {
    static ref LETTERS: Vec<char> = "abcdefg".chars().collect();
    static ref DIGITS: HashMap<BTreeSet<char>, usize> =
        ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]
            .into_iter()
            .enumerate()
            .map(|(index, letter)| (letter.chars().collect(), index))
            .collect();
}

pub fn solve_connections(entries: Vec<Display>, outputs: Vec<Display>) -> usize {
    LETTERS
        .iter()
        .copied()
        .permutations(7)
        .find_map(|permutation| {
            let translation = permutation
                .iter()
                .copied()
                .enumerate()
                .map(|(index, letter)| (letter, LETTERS[index]))
                .collect::<HashMap<_, _>>();

            let sets = entries
                .iter()
                .chain(outputs.iter())
                .map(|display| {
                    display
                        .iter()
                        .map(|key| translation[&key])
                        .collect::<BTreeSet<_>>()
                })
                .collect::<Vec<_>>();

            if !sets.iter().all(|set| DIGITS.contains_key(set)) {
                return None;
            }

            let result = sets[10..14]
                .iter()
                .map(|set| DIGITS[set].to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            Some(result)
        })
        .expect("should be one result")
}

impl Display {
    /// The character representing each "on" segment in this [Display]
    pub fn iter(&self) -> impl Iterator<Item = char> {
        self.buffer
            .clone()
            .into_iter()
            .zip(LETTERS.iter().copied())
            .filter_map(|(segment, letter)| matches!(segment, Segment::On).then(|| letter))
    }
}
