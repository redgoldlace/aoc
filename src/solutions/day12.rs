aoc!(day = 12, part = 1);

use std::collections::{HashMap, HashSet};

#[transform]
fn transform(input: _) -> HashMap<Cave<'a>, HashSet<Cave<'a>>> {
    let mut result = HashMap::<_, HashSet<_>>::new();

    input
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .map(|(start, end)| (Cave::new(start), Cave::new(end)))
        .for_each(|(start, end)| {
            result.entry(start).or_default().insert(end);
            result.entry(end).or_default().insert(start);
        });

    result
}

#[solve]
fn solve(input: _) -> usize {
    distinct_paths(&input).len()
}

pub fn distinct_paths<'a>(
    cave_mapping: &HashMap<Cave<'a>, HashSet<Cave<'a>>>,
) -> Vec<Vec<Cave<'a>>> {
    let mut paths = Vec::<Vec<Cave>>::new();
    let mut results = Vec::<Vec<Cave>>::new();

    paths.push(vec![Cave::Start]);

    while let Some(path) = paths.pop() {
        let last = path.last().copied().unwrap();

        if last.is_end() {
            results.push(path);
            continue;
        }

        for next in cave_mapping[&last].iter().copied() {
            if next.is_start() || (next.is_small() && path.contains(&next)) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(next);

            paths.push(new_path);
        }
    }

    results
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
    Start,
    End,
}

impl<'a> Cave<'a> {
    pub fn new(identifier: &'a str) -> Self {
        if identifier == "start" {
            Cave::Start
        } else if identifier == "end" {
            Cave::End
        } else if identifier.chars().all(char::is_uppercase) {
            Cave::Big(identifier)
        } else {
            Cave::Small(identifier)
        }
    }

    pub fn is_big(self) -> bool {
        matches!(self, Cave::Big(_))
    }

    pub fn is_small(self) -> bool {
        matches!(self, Cave::Small(_))
    }

    pub fn is_start(self) -> bool {
        matches!(self, Cave::Start)
    }

    pub fn is_end(self) -> bool {
        matches!(self, Cave::End)
    }
}
