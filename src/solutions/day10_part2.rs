aoc!(day = 10, part = 2);

use super::day10::{Delimiter, Position, Symbol};

#[transform]
fn transform(input: _) -> Vec<Vec<Symbol>> {
    input
        .lines()
        .map(|line| line.chars().map(Symbol::new).collect())
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    let mut scores = input
        .iter()
        .filter_map(|sequence| match_symbols(sequence))
        .map(|stack| {
            stack
                .into_iter()
                .rev()
                .fold(0, |score, symbol| score * 5 + symbol.1.closing_score())
        })
        .collect::<Vec<usize>>();

    scores.sort();

    scores[scores.len() / 2]
}

pub fn match_symbols(sequence: &[Symbol]) -> Option<Vec<Symbol>> {
    let mut stack: Vec<Symbol> = Vec::new();

    for symbol @ Symbol(position, delimiter) in sequence.iter().copied() {
        if position == Position::Opening {
            stack.push(symbol);
            continue;
        }

        let Symbol(_, opening_delimiter) = stack.pop().expect("stack underrun");

        if delimiter != opening_delimiter {
            return None;
        }
    }

    (stack.len() > 0).then(|| stack)
}

impl Delimiter {
    pub fn closing_score(self) -> usize {
        match self {
            Delimiter::Parenthesis => 1,
            Delimiter::SquareBracket => 2,
            Delimiter::CurlyBrace => 3,
            Delimiter::AngleBracket => 4,
        }
    }
}
