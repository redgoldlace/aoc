use super::day4::{Cell, Grid};
use crate::prelude::*;
use std::collections::HashSet;

impl<'a> Solution<'a> for Day<4, { Part::Two }> {
    type Transformed = (Vec<usize>, Vec<Grid<Cell>>);
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        Day::<4, { Part::One }>::transform(input)
    }

    fn solve((numbers, mut boards): Self::Transformed) -> Self::Result {
        let (last_called, winning_index) = last_to_win(&numbers, &mut boards)
            .expect("expected at least one winner. this is bingo. c'mon.");

        let unmarked = boards[winning_index]
            .iter()
            .filter_map(|cell| (!cell.marked).then(|| cell.number))
            .sum::<usize>();

        unmarked * last_called
    }
}

pub fn last_to_win(numbers: &[usize], boards: &mut [Grid<Cell>]) -> Option<(usize, usize)> {
    let mut live = (0..boards.len()).collect::<HashSet<_>>();

    for &number in numbers {
        let cloned = live.clone();
        let live_boards = boards
            .iter_mut()
            .enumerate()
            .filter(move |(index, _)| cloned.contains(index));

        for (index, board) in live_boards {
            let position = match board.mark(number) {
                Some(position) => position,
                None => continue,
            };

            if !board.check(position) {
                continue;
            }

            if live.len() == 1 {
                return Some((number, index));
            } else {
                live.remove(&index);
            }
        }
    }

    None
}
