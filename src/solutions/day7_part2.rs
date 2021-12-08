aoc!(day = 7, part = 2);

#[transform]
fn transform(input: _) -> Vec<usize> {
    <day!(7)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    (0_usize..=input.iter().copied().max().unwrap())
        .map(|position| {
            input
                .iter()
                .copied()
                .map(|crab| difference(crab, position))
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

// Part 2 is very difficult
pub fn difference(a: usize, b: usize) -> usize {
    let difference = super::day7::difference(a, b);

    (difference * (difference + 1)) / 2
}
