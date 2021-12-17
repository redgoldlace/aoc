aoc!(day = 17, part = 2);

use super::day17::{range_product, simulate, Target};

#[transform]
fn transform(input: _) -> Target {
    <day!(17)>::transform(input)
}

#[solve]
fn solve(input: _) -> isize {
    let x_range = 0..=*input.x.end();
    let y_range = *input.y.start()..=input.y.start().abs();

    let mut possibilities = 0;

    for velocity in range_product(x_range, y_range) {
        if simulate(&input, velocity).is_none() {
            continue;
        };

        possibilities += 1;
    }

    possibilities
}
