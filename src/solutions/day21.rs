aoc!(day = 21, part = 1);

#[transform]
fn transform(input: _) -> Vec<Player> {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .map(Player::new)
        .collect()
}

#[solve]
fn solve(input: _) -> usize {
    let mut players = input;

    let mut rolls = 0;
    let mut dice = std::iter::repeat(1..=100).flatten().inspect(|_| rolls += 1);

    // Yuck.

    'outer: loop {
        for player in players.iter_mut() {
            let roll = dice.by_ref().take(3).sum();
            *player = player.take_turn(roll);

            if player.wins() {
                break 'outer;
            }
        }
    }

    let losing_score = players
        .iter()
        .min_by_key(|player| player.score)
        .unwrap()
        .score;

    losing_score * rolls
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    pub(crate) position: usize,
    pub(crate) score: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    pub fn take_turn(mut self, roll: usize) -> Self {
        self.position = (self.position + roll - 1) % 10 + 1;
        self.score += self.position;

        self
    }

    pub fn wins(&self) -> bool {
        self.score >= 1000
    }
}
