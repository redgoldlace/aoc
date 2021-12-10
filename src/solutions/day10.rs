aoc!(day = 10, part = 1);

#[transform]
fn transform(input: _) -> Vec<Vec<Symbol>> {
    input
        .lines()
        .map(|line| line.chars().map(Symbol::new).collect())
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    input
        .iter()
        .filter_map(|sequence| match_symbols(sequence))
        .map(Delimiter::score)
        .sum()
}

pub fn match_symbols(sequence: &[Symbol]) -> Option<Delimiter> {
    let mut stack: Vec<Symbol> = Vec::new();

    for symbol @ Symbol(position, delimiter) in sequence.iter().copied() {
        if position == Position::Opening {
            stack.push(symbol);
            continue;
        }

        let Symbol(_, opening_delimiter) = stack.pop().expect("stack underrun");

        if delimiter != opening_delimiter {
            return Some(delimiter);
        }
    }

    // We can ignore incomplete inputs for now

    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Delimiter {
    Parenthesis,
    CurlyBrace,
    SquareBracket,
    AngleBracket,
}

impl Delimiter {
    pub fn score(self) -> usize {
        // These sure are some magic numbers
        match self {
            Delimiter::Parenthesis => 3,
            Delimiter::SquareBracket => 57,
            Delimiter::CurlyBrace => 1197,
            Delimiter::AngleBracket => 25137,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy)]
pub struct Symbol(pub Position, pub Delimiter);

impl Symbol {
    pub fn new(character: char) -> Self {
        match character {
            '(' => Symbol(Position::Opening, Delimiter::Parenthesis),
            ')' => Symbol(Position::Closing, Delimiter::Parenthesis),
            '{' => Symbol(Position::Opening, Delimiter::CurlyBrace),
            '}' => Symbol(Position::Closing, Delimiter::CurlyBrace),
            '[' => Symbol(Position::Opening, Delimiter::SquareBracket),
            ']' => Symbol(Position::Closing, Delimiter::SquareBracket),
            '<' => Symbol(Position::Opening, Delimiter::AngleBracket),
            '>' => Symbol(Position::Closing, Delimiter::AngleBracket),
            _ => unreachable!("invalid character literal in test input"),
        }
    }
}
