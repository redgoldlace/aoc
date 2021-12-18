aoc!(day = 18, part = 1);

use logos::Logos;

#[transform]
fn transform(input: _) -> Vec<Vec<Pair>> {
    input.trim().lines().map(parse).collect(    )
}

#[solve]
fn solve(input: _) -> usize {
    magnitude(input.into_iter().reduce(|a, b| add(&a, &b)).unwrap())
}

pub fn parse(input: &str) -> Vec<Pair> {
    let mut lexer = Token::lexer(input.trim());
    let mut stack = Vec::new();
    let mut depth = 0;

    while let Some(token) = lexer.next() {
        match token {
            Token::Number(value) => stack.push((value, depth)),
            Token::OpeningBracket => depth += 1,
            Token::ClosingBracket => depth -= 1,
            _ => continue,
        }
    }

    stack
}

pub fn magnitude(mut buffer: Vec<Pair>) -> usize {
    while buffer.len() > 1 {
        let Some(max_depth) = buffer.iter().map(|&(_, depth)| depth).max() else {
            break
        };

        let Some(index) = buffer.iter().position(|&(_, depth)| depth == max_depth) else {
            break
        };

        let (left_index, right_index) = (index, index + 1);

        buffer[left_index].0 = buffer[left_index].0 * 3 + buffer[right_index].0 * 2;
        buffer[left_index].1 -= 1;

        buffer.remove(right_index);
    }

    buffer[0].0
}

#[test]
fn test_magnitude() {
    let inputs = [
        ("[[9,1],[1,9]]", 129),
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
    ];

    for (input, answer) in inputs {
        assert_eq!(magnitude(parse(input)), answer)
    }
}

pub fn add(first: &[Pair], second: &[Pair]) -> Vec<Pair> {
    let mut buffer = first
        .into_iter()
        .chain(second)
        .map(|&(value, depth)| (value, depth + 1))
        .collect();

    reduce(&mut buffer);

    buffer
}

pub fn reduce(buffer: &mut Vec<Pair>) {
    loop {
        if let Some(index) = buffer.iter().position(|&(_, depth)| depth > 4) {
            explode(index, buffer);
        } else if let Some(index) = buffer.iter().position(|&(value, _)| value >= 10) {
            split(index, buffer);
        } else {
            break;
        };
    }
}

pub fn explode(index: usize, buffer: &mut Vec<Pair>) {
    let (left_index, right_index) = (index, index + 1);

    assert_eq!(
        buffer[left_index].1, buffer[right_index].1,
        "tried to explode invalid pair"
    );

    let (left, current_depth) = buffer[index];
    let (right, _) = buffer[index + 1];

    let bounds = 1..buffer.len() - 1;

    if bounds.contains(&left_index) {
        buffer[left_index - 1].0 += left;
    }

    if bounds.contains(&right_index) {
        buffer[right_index + 1].0 += right;
    }

    // Now remove the pair and replace with a 0
    buffer.remove(right_index);
    buffer[left_index] = (0, current_depth - 1);
}

pub fn split(index: usize, buffer: &mut Vec<Pair>) {
    let (current, current_depth) = buffer[index];

    let magic_number = current as f64 / 2.0;
    let new_depth = current_depth + 1;

    buffer[index] = (magic_number.floor() as usize, new_depth);
    buffer.insert(index + 1, (magic_number.ceil() as usize, new_depth));
}

pub type Pair = (usize, usize);

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(usize),
    #[token(",")]
    Comma,
    #[token("[")]
    OpeningBracket,
    #[token("]")]
    ClosingBracket,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}
