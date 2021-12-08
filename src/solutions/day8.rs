aoc!(day = 8, part = 1);

use std::{collections::HashSet, convert::Infallible, str::FromStr};

#[transform]
fn transform(input: _) -> Vec<(Vec<Display>, Vec<Display>)> {
    input
        .lines()
        .map(|line| {
            let mut patterns = line.split('|').map(|half| {
                half.split_whitespace()
                    .map(|segment| segment.trim().parse::<Display>().unwrap())
                    .collect::<Vec<_>>()
            });

            (patterns.next().unwrap(), patterns.next().unwrap())
        })
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    input
        .iter()
        .flat_map(|(_entries, output)| output.iter())
        .map(|display| matches!(display.on(), 2 | 3 | 4 | 7) as usize)
        .sum()
}

#[derive(Debug, Clone, Copy)]
pub enum Segment {
    On,
    Off,
}

#[derive(Debug, Clone)]
pub struct Display {
    pub(crate) buffer: [Segment; 7],
}

impl Display {
    /// The total number of "on" segments in this [Display]
    pub fn on(&self) -> usize {
        self.buffer
            .iter()
            .map(|segment| matches!(segment, Segment::On) as usize)
            .sum::<usize>()
    }
}

impl FromStr for Display {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indexes = s
            .chars()
            // This is a cheat to give us an index of 0..8
            .map(|character| character as usize - 97)
            .collect::<HashSet<usize>>();

        let mut buffer = [Segment::Off; 7];

        for i in 0..8 {
            if !indexes.contains(&i) {
                continue;
            }

            buffer[i] = Segment::On;
        }

        Ok(Display { buffer })
    }
}
