aoc!(day = 1, part = 2);

#[transform]
fn transform(input: _) -> Vec<usize> {
    <day!(1)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    input
        .windows(3)
        .zip(input.windows(3).skip(1))
        .map(|(a, b)| (a.iter().copied(), b.iter().copied()))
        .map(|(a, b)| (b.sum::<usize>() > a.sum::<usize>()) as usize)
        .sum()
}
