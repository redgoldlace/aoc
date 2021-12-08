aoc!(day = 4, part = 2);

use super::day4::{Cell, Grid};
use std::collections::HashSet;

#[transform]
fn transform(input: _) -> (Vec<usize>, Vec<Grid<Cell>>) {
    <day!(4)>::transform(input)
}

#[solve]
fn solve((numbers, mut boards): _) -> usize {
    let (last_called, winning_index) = last_to_win(&numbers, &mut boards)
        .expect("expected at least one winner. this is bingo. c'mon.");

    let unmarked = boards[winning_index]
        .iter()
        .filter_map(|cell| (!cell.marked).then(|| cell.number))
        .sum::<usize>();

    unmarked * last_called
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
