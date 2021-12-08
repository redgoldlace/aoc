aoc!(day = 3, part = 1);

#[transform]
fn transform(input: _) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap() != 0)
                .collect::<Vec<bool>>()
        })
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    let columns = input[0].len();

    let mut gamma = Vec::with_capacity(columns);
    let mut epsilon = Vec::with_capacity(columns);

    for i in 0..columns {
        let ones = column(&input, i).map(Into::<usize>::into).sum::<usize>();
        let zeroes = input.len() - ones;

        gamma.push(ones > zeroes);
        epsilon.push(ones < zeroes);
    }

    bits_to_n(&gamma) * bits_to_n(&epsilon)
}

pub fn column<'a>(slice: &'a [Vec<bool>], index: usize) -> impl Iterator<Item = bool> + 'a {
    slice.iter().map(move |row| row[index])
}

pub fn bits_to_n(slice: &[bool]) -> usize {
    slice.iter().fold(0, |n, &bit| ((n * 2) + bit as usize))
}
