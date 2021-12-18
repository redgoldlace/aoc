aoc!(day = 16, part = 1);

#[transform]
fn transform(input: _) -> Vec<bool> {
    let mut buffer = Vec::with_capacity(input.trim().len() * 4);

    // Thank you macro span/scoping rules very cool
    use crate::bits;

    buffer.extend(input.trim().chars().flat_map(|hex| match hex {
        '0' => bits![0 0 0 0],
        '1' => bits![0 0 0 1],
        '2' => bits![0 0 1 0],
        '3' => bits![0 0 1 1],
        '4' => bits![0 1 0 0],
        '5' => bits![0 1 0 1],
        '6' => bits![0 1 1 0],
        '7' => bits![0 1 1 1],
        '8' => bits![1 0 0 0],
        '9' => bits![1 0 0 1],
        'A' => bits![1 0 1 0],
        'B' => bits![1 0 1 1],
        'C' => bits![1 1 0 0],
        'D' => bits![1 1 0 1],
        'E' => bits![1 1 1 0],
        'F' => bits![1 1 1 1],
        _ => unreachable!(),
    }));

    buffer
}

#[solve]
fn solve(input: _) -> usize {
    let mut stream = Stream::new(&input);
    let result = parse_packet(&mut stream).unwrap();

    result
        .unfold()
        .iter()
        .map(|packet| packet.version as usize)
        .sum()
}

pub fn parse_packet(stream: &mut Stream<bool>) -> Option<Packet> {
    let version = stream.take(3)?.unbinary() as u8;
    let type_id = stream.take(3)?.unbinary() as u8;

    let body = match type_id {
        4 => parse_literal(stream),
        _ => parse_operator(stream),
    }?;

    let result = Packet {
        version,
        type_id,
        body,
    };

    Some(result)
}

pub fn parse_operator(stream: &mut Stream<bool>) -> Option<PacketBody> {
    let subpacket_type: Subpacket = stream.take_one().copied()?.into();

    let to_read = match subpacket_type {
        Subpacket::Count => 11,
        Subpacket::Width => 15,
    };

    let magic_number = stream.take(to_read)?.unbinary();

    let packets = match subpacket_type {
        Subpacket::Count => parse_packets(stream, magic_number),
        Subpacket::Width => parse_packets_by_width(stream, magic_number),
    };

    packets.map(PacketBody::Operator)
}

pub enum Subpacket {
    Count,
    Width,
}

impl From<bool> for Subpacket {
    fn from(value: bool) -> Self {
        match value {
            true => Subpacket::Count,
            false => Subpacket::Width,
        }
    }
}

pub fn parse_packets_by_width(stream: &mut Stream<bool>, length: usize) -> Option<Vec<Packet>> {
    let mut packets = Vec::new();
    let marker = stream.mark();

    while marker.measure(&stream) < length {
        packets.push(parse_packet(stream)?)
    }

    // Make sure we haven't somehow overshot
    assert_eq!(marker.measure(&stream), length);

    Some(packets)
}

pub fn parse_packets(stream: &mut Stream<bool>, n: usize) -> Option<Vec<Packet>> {
    (0..n).map(|_| parse_packet(stream)).collect()
}

pub fn parse_literal(stream: &mut Stream<bool>) -> Option<PacketBody> {
    let mut value = 0;

    while let [first, rest @ ..] = stream.take(5)? {
        value <<= 4;
        value |= rest.unbinary() as u64;

        if !first {
            break;
        }
    }

    Some(PacketBody::Literal(value))
}

#[derive(Debug)]
pub struct Packet {
    pub (crate) version: u8,
    pub (crate) type_id: u8,
    pub (crate) body: PacketBody,
}

impl Packet {
    pub fn unfold(&self) -> Vec<&Packet> {
        fn unfold<'a>(packet: &'a Packet, buffer: &mut Vec<&'a Packet>) {
            buffer.push(packet);

            if let PacketBody::Operator(ref subpackets) = packet.body {
                for subpacket in subpackets {
                    unfold(subpacket, buffer);
                }
            }
        }

        let mut buffer = Vec::new();
        unfold(self, &mut buffer);

        buffer
    }
}

#[derive(Debug)]
pub enum PacketBody {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[macro_export]
macro_rules! bits {
    (@emit 0) => {
        false
    };
    (@emit 1) => {
        true
    };
    ($($lit:tt)*) => {
        [$($crate::bits!(@emit $lit)),*]
    };
}

pub struct Stream<'a, T> {
    pub(crate) buffer: &'a [T],
}

impl<'a, T> Stream<'a, T> {
    pub fn new(buffer: &'a [T]) -> Self {
        Self { buffer }
    }

    pub fn take(&mut self, n: usize) -> Option<&'a [T]> {
        if n > self.buffer.len() {
            self.buffer = &[];
            None
        } else {
            let (chunk, rest) = self.buffer.split_at(n);
            self.buffer = rest;
            Some(chunk)
        }
    }

    pub fn take_one(&mut self) -> Option<&'a T> {
        let result = self.take(1)?;

        Some(&result[0])
    }

    pub fn mark(&self) -> Marker<'a, T> {
        Marker(self.buffer)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Marker<'a, T>(&'a [T]);

impl<'a, T> Marker<'a, T> {
    pub fn measure(self, stream: &Stream<'a, T>) -> usize {
        assert_eq!(
            self.0.as_ptr_range().end,
            stream.buffer.as_ptr_range().end,
            "marker comes from different stream"
        );

        // We also need to make sure that the laws of causality haven't been violated.
        assert!(
            stream.buffer.len() <= self.0.len(),
            "stream is earlier than marker"
        );

        self.0.len() - stream.buffer.len()
    }
}

pub trait UnbinaryExt {
    fn unbinary(self) -> usize;
}

impl UnbinaryExt for &[bool] {
    fn unbinary(self) -> usize {
        self.into_iter()
            .copied()
            .fold(0, |n, bit| (n << 1) | bit as usize)
    }
}
