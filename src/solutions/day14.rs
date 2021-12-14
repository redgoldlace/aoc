aoc!(day = 14, part = 1);

use std::collections::HashMap;
use counter::Counter;

#[transform]
fn transform(input: _) -> (Vec<char>, HashMap<Pair, char>) {
    let mut lines = input.lines();

    let template: Vec<char> = lines.next().unwrap().chars().collect();
    
    let insertions = lines
        .skip(1)
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();

            let mut components = left.chars();

            (
                (components.next().unwrap(), components.next().unwrap()),
                right.chars().next().unwrap(),
            )
        })
        .collect();

    (template, insertions)
}

#[solve]
fn solve(input: _) -> usize {
    let (template, replacements) = input;
    let mut frequency = collapse(template);

    for _ in 0..10 {
        frequency = step(frequency, &replacements);
    }

    element_delta(&frequency)
}

pub fn element_delta(frequency: &HashMap<Pair, usize>) -> usize {
    let mut counts = Counter::<char>::new();

    for ((first, second), &count) in frequency {
        counts[first] += count;
        counts[second] += count;
    }

    let most_common = counts.most_common();

    let max = most_common.first().unwrap().1;
    let min = most_common.last().unwrap().1;

    max / 2 - min / 2
}

pub fn collapse(template: Vec<char>) -> HashMap<Pair, usize> {
    let mut map = HashMap::<Pair, usize>::new();

    for window in template.windows(2) {
        *map.entry((window[0], window[1])).or_default() += 1;
    }

    map
}

pub type Pair = (char, char);

pub fn step(
    frequency: HashMap<Pair, usize>,
    replacements: &HashMap<Pair, char>,
) -> HashMap<Pair, usize> {
    let mut result: HashMap<Pair, usize> = HashMap::new();

    for (&(first, second), &count) in frequency.iter() {
        let pair = (first, second);

        match replacements.get(&pair).copied() {
            Some(new) => {
                *result.entry((first, new)).or_default() += count;
                *result.entry((new, second)).or_default() += count;
            }
            None => *result.entry(pair).or_default() += count,
        }
    }

    result
}
