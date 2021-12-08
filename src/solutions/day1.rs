aoc!(day = 1, part = 1);

#[transform]
fn transform(input: _) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    input
        .windows(2)
        .map(|window| (window[0] < window[1]) as usize)
        .sum()
}
