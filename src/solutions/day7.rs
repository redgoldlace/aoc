aoc!(day = 7, part = 1);

#[transform]
fn transform(input:_) -> Vec<usize> {
    input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
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

pub fn difference(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as _
}
