use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One = 1,
    Two,
}

pub struct Day<const N: usize, const PART: Part>;

pub trait Solution<'a> {
    type Transformed;
    type Result;

    fn transform(input: &'a str) -> Self::Transformed;
    fn solve(input: Self::Transformed) -> Self::Result;

    fn run(input: &'a str) -> Box<dyn Display + 'a>
    where
        Self::Result: Display + 'a,
    {
        Box::new(Self::solve(Self::transform(input)))
    }
}

// This exposes a generated function that looks a bit like the below:
//
// fn run<'a>(day: u8, part: Part, input: &'a str) -> Option<Box<dyn std::fmt::Display + 'a>> {
//     ...
// }
//
// This function is used to delegate to the individual implementations of the `Solution` trait that are defined in the
// `solutions` directory.
//
// It additionally exposes a constant `IMPLEMENTED`, which is a slice of `(u8, Part)` pairs representing which solutions
// have implementations. This slice is sorted in ascending order, and is used to determine the latest solution when
// running `aoc run latest`.

include!(concat!(env!("OUT_DIR"), "/run.rs"));

#[macro_export]
macro_rules! day {
    ($number:literal) => {
        $crate::solution::Day::<$number, { $crate::part!(1) }>
    };
    ($number:literal part 1) => {
        $crate::solution::Day::<$number, { $crate::part!(1) }>
    };
    ($number:literal part 2) => {
        $crate::solution::Day::<$number, { $crate::part!(2) }>
    };
}

#[macro_export]
macro_rules! part {
    (1) => {
        $crate::solution::Part::One
    };
    (2) => {
        $crate::solution::Part::Two
    };
}
