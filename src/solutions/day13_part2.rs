aoc!(day = 13, part = 2);

use std::fmt::{Display, Write};

use super::day13::{Fold, Paper};

#[transform]
fn transform(input: _) -> (Paper, Vec<Fold>) {
    <day!(13)>::transform(input)
}

#[solve]
fn solve(input: _) -> String {
    let (mut paper, instructions) = input;

    for instruction in instructions {
        paper.fold(instruction);
    }

    paper.to_string()
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(match self.points.get(&(x, y)) {
                    Some(_) => '#',
                    None => ' ',
                })?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
