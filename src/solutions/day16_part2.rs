aoc!(day = 16, part = 2);

use super::day16::{parse_packet, Packet, PacketBody, Stream};
use std::cmp::Ordering;

#[transform]
fn transform(input: _) -> Vec<bool> {
    <day!(16)>::transform(input)
}

#[solve]
fn solve(input: _) -> usize {
    let mut stream = Stream::new(&input);

    parse_packet(&mut stream).unwrap().evaluate()
}

pub type EvalFn = fn(&[Packet]) -> usize;

#[inline]
pub fn eval<'a>(packets: impl IntoIterator<Item = &'a Packet>) -> impl Iterator<Item = usize> {
    packets.into_iter().map(Packet::evaluate)
}

#[inline]
pub fn cmp_eval(packets: &[Packet], desired: Ordering) -> usize {
    (packets[0].evaluate().cmp(&packets[1].evaluate()) == desired) as usize
}

pub const VTABLE: [EvalFn; 8] = [
    |packets| eval(packets).sum(),
    |packets| eval(packets).product(),
    |packets| eval(packets).min().unwrap(),
    |packets| eval(packets).max().unwrap(),
    |_| panic!("literals have no vtable entry"),
    |packets| cmp_eval(packets, Ordering::Greater),
    |packets| cmp_eval(packets, Ordering::Less),
    |packets| cmp_eval(packets, Ordering::Equal),
];

impl Packet {
    pub fn evaluate(&self) -> usize {
        match self.body {
            PacketBody::Literal(value) => value as usize,
            PacketBody::Operator(ref subpackets) => VTABLE[self.type_id as usize](subpackets),
        }
    }
}
