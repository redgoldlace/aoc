aoc!(day = 12, part = 2);

use super::day12::Cave;
use std::collections::{HashMap, HashSet};

#[transform]
fn transform(input: _) -> HashMap<Cave<'a>, HashSet<Cave<'a>>> {
    <day!(12)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    distinct_paths_with_revisiting(&input).len()
}

pub fn distinct_paths_with_revisiting<'a>(
    cave_mapping: &HashMap<Cave<'a>, HashSet<Cave<'a>>>,
) -> Vec<Vec<Cave<'a>>> {
    let mut paths = Vec::<(bool, Vec<Cave>)>::new();
    let mut results = Vec::<Vec<Cave>>::new();

    paths.push((false, vec![Cave::Start]));

    while let Some((has_revisited, path)) = paths.pop() {
        let last = path.last().copied().unwrap();

        if last.is_end() {
            results.push(path);
            continue;
        }

        for next in cave_mapping[&last].iter().copied() {
            let visit_count = path.iter().copied().filter(|&cave| cave == next).count();
            let revisited_this = next.is_small() && visit_count == 1;

            if next.is_start() || (next.is_small() && has_revisited && visit_count >= 1) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(next);

            paths.push((has_revisited || revisited_this, new_path));
        }
    }

    results
}
