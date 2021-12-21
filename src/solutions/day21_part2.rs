aoc!(day = 21, part = 2);

use super::day21::Player;
use indexmap::IndexMap;
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

#[transform]
fn transform(input: _) -> Vec<Player> {
    <day!(21)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    let mut universes = UniverseMap::with_capacity_and_hasher(200, Default::default());

    universes.insert((input[0], input[1]), 1);

    let mut win_counts = [0, 0];

    while let Some(((player_one, player_two), value)) = universes.pop() {
        for &roll in ROLLS.iter() {
            let player_one = player_one.take_turn(roll);

            if player_one.quantum_wins() {
                win_counts[0] += value;
                continue;
            }

            for &roll in ROLLS.iter() {
                let player_two = player_two.take_turn(roll);

                if player_two.quantum_wins() {
                    win_counts[1] += value;
                    continue;
                }

                *universes.entry((player_one, player_two)).or_default() += value;
            }
        }
    }

    win_counts[0].max(win_counts[1])
}

impl Player {
    pub fn quantum_wins(&self) -> bool {
        self.score >= 21
    }
}

pub const ROLLS: [usize; 27] = [
    3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
];

pub type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;
pub type UniverseMap = FxIndexMap<(Player, Player), usize>;
