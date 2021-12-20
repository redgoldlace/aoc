aoc!(day = 19, part = 1);

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[transform]
fn transform(input: &str) -> Vec<Scanner> {
    let mut lines = input.lines();
    let mut scanners = Vec::new();

    // The first line of a block is a scanner. We can ignore it.
    while let Some(_) = lines.next() {
        let scanner: Scanner = lines
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .map(|line| {
                line.trim()
                    .split(',')
                    .map(|line| line.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        scanners.push(scanner);
    }

    scanners
}

#[solve]
fn solve(mut input: _) -> usize {
    align(&mut input);

    let beacons: FxHashSet<_> = input
        .iter()
        .flat_map(|scanner| scanner.signals.iter())
        .map(|signal| signal.position)
        .collect();

    beacons.len()
}

pub fn align(scanners: &mut Vec<Scanner>) {
    let mut locked = vec![0_usize];

    scanners[0].position = Some((0, 0, 0));

    while locked.len() < scanners.len() {
        let permutations = (0..scanners.len())
            .permutations(2)
            .map(|items| (items[0], items[1]));

        for (a, b) in permutations {
            let not_visited = !locked.iter().contains(&a);
            let visited_already = locked.iter().contains(&b);

            if a == b || not_visited || visited_already {
                continue;
            }

            let Some(overlap) = scanners[a].overlapping(&scanners[b]) else {
                continue;
            };

            assert_ne!(a, b);

            let ptr_a = &mut scanners[a] as *mut Scanner;
            let ptr_b = &mut scanners[b] as *mut Scanner;

            // SAFETY: `a` and `b` are disjoint indexes, so these reborrows do not alias.
            unsafe { Scanner::align(&mut *ptr_a, &mut *ptr_b, overlap) }

            locked.push(b);
        }
    }
}

pub type Triple<T = isize> = (T, T, T);

pub struct Signal {
    pub(crate) position: Triple,
    relatives: FxHashMap<usize, Identity>,
    id: usize,
}

impl Signal {
    pub fn matches(&self, other: &Signal) -> Vec<Match> {
        let mut buffer = Vec::new();

        for (&index, &relative) in self.relatives.iter() {
            let found = other
                .relatives
                .iter()
                .find_map(|(&id, &item)| (item == relative).then(|| id));

            let Some(other_index) = found else {
                continue;
            };

            buffer.push(Match {
                from: index,
                to: other_index,
            })
        }

        buffer
    }

    pub fn link(&mut self, other: &mut Self) {
        let (x1, y1, z1) = self.position;
        let (x2, y2, z2) = other.position;

        let distances = ((x1 - x2).abs(), (y1 - y2).abs(), (z1 - z2).abs());
        let identity = Identity::new(distances);

        other.relatives.insert(self.id, identity);
        self.relatives.insert(other.id, identity);
    }
}

pub struct Match {
    from: usize,
    to: usize,
}

pub struct Scanner {
    pub(crate) signals: Vec<Signal>,
    pub(crate) position: Option<Triple>,
}

impl FromIterator<Triple> for Scanner {
    fn from_iter<T: IntoIterator<Item = Triple>>(iter: T) -> Self {
        let mut scanner = Scanner::new();

        for position in iter {
            scanner.insert(position)
        }

        scanner
    }
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
            position: None,
        }
    }

    pub fn insert(&mut self, position: Triple) {
        let mut new_signal = Signal {
            position,
            relatives: FxHashMap::default(),
            id: self.signals.len(),
        };

        for signal in self.signals.iter_mut() {
            signal.link(&mut new_signal);
        }

        self.signals.push(new_signal);
    }

    pub fn overlapping<'a>(&'a self, other: &'a Scanner) -> Option<Overlap> {
        let signals = other.signals.iter().enumerate().flat_map(|other_signal| {
            self.signals
                .iter()
                .enumerate()
                .map(move |signal| (other_signal, signal))
        });

        for ((there_id, there), (here_id, here)) in signals {
            let matches = there.matches(here);

            if matches.len() >= 11 {
                return Some(Overlap {
                    there_id,
                    here_id,
                    matches,
                });
            }
        }

        None
    }

    pub fn align(&mut self, other: &mut Self, overlap: Overlap) {
        for matched in overlap.matches {
            if matched.from == 0 {
                continue;
            }

            let relative_here = &self.signals[matched.to];
            let relative_there = &other.signals[matched.from];

            let distance1 = sub(
                self.signals[overlap.here_id].position,
                relative_here.position,
            );

            let distance2 = sub(
                other.signals[overlap.there_id].position,
                relative_there.position,
            );

            // X == Y, X == Z, Y == Z
            let (x_to_y, x_to_z, y_to_z) = (
                distance1.0 == distance1.1,
                distance1.0 == distance1.2,
                distance1.1 == distance1.2,
            );

            if x_to_y || x_to_z || y_to_z {
                continue;
            }

            let reoriented_x = reorient(distance1.0, distance2);
            let reoriented_y = reorient(distance1.1, distance2);
            let reoriented_z = reorient(distance1.2, distance2);

            for signal in other.signals.iter_mut() {
                let position = signal.position;

                signal.position = (
                    sum(mul(position, reoriented_x)),
                    sum(mul(position, reoriented_y)),
                    sum(mul(position, reoriented_z)),
                );
            }

            other.position = Some(sub(
                self.signals[overlap.here_id].position,
                other.signals[overlap.there_id].position,
            ));

            for signal in other.signals.iter_mut() {
                signal.position = add(signal.position, other.position.unwrap())
            }

            // Whew.
            break;
        }
    }
}

pub fn reorient(value: isize, (x, y, z): Triple) -> Triple {
    (sign(value, x), sign(value, y), sign(value, z))
}

pub fn sign(a: isize, b: isize) -> isize {
    match (a, b) {
        (a, b) if a == b => 1,
        (a, b) if a == -b => -1,
        _ => 0,
    }
}

pub fn sub((x1, y1, z1): Triple, (x2, y2, z2): Triple) -> Triple {
    (x1 - x2, y1 - y2, z1 - z2)
}

pub fn add((x1, y1, z1): Triple, (x2, y2, z2): Triple) -> Triple {
    (x1 + x2, y1 + y2, z1 + z2)
}

pub fn mul((x1, y1, z1): Triple, (x2, y2, z2): Triple) -> Triple {
    (x1 * x2, y1 * y2, z1 * z2)
}

pub fn sum((x, y, z): Triple) -> isize {
    x + y + z
}

pub struct Overlap {
    there_id: usize,
    here_id: usize,
    matches: Vec<Match>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identity {
    distance: isize,
    min: isize,
    max: isize,
}

impl Identity {
    pub fn new((x, y, z): Triple) -> Self {
        let (abs_x, abs_y, abs_z) = (x.abs(), y.abs(), z.abs());

        Self {
            distance: x.pow(2) + y.pow(2) + z.pow(2),
            min: abs_x.min(abs_y).min(abs_z),
            max: abs_x.max(abs_y).max(abs_z),
        }
    }
}
