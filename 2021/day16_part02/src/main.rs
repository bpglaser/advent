use bitvec::prelude::*;
use std::{env::args, fs::File, io::Read};

#[derive(Debug)]
enum Packet {
    Literal(usize),
    Operator {
        id: usize,
        children: Vec<VersionedPacket>,
    },
}

impl Packet {
    fn eval(&self) -> usize {
        match self {
            Packet::Literal(i) => *i,
            Packet::Operator { id, children } => {
                let fold = |init: usize, op: Box<dyn Fn(usize, usize) -> usize>| {
                    children
                        .into_iter()
                        .map(|vp| vp.packet.eval())
                        .fold(init, op)
                };
                let eval_pair = |op: Box<dyn Fn(usize, usize) -> usize>| {
                    assert!(children.len() == 2);
                    op(children[0].packet.eval(), children[1].packet.eval())
                };
                match id {
                    0 => fold(0, Box::new(|a: usize, b: usize| a + b)),
                    1 => fold(1, Box::new(|a: usize, b: usize| a * b)),
                    2 => fold(usize::MAX, Box::new(std::cmp::min)),
                    3 => fold(0, Box::new(std::cmp::max)),
                    5 => eval_pair(Box::new(|a: usize, b: usize| if a > b { 1 } else { 0 })),
                    6 => eval_pair(Box::new(|a: usize, b: usize| if a < b { 1 } else { 0 })),
                    7 => eval_pair(Box::new(|a: usize, b: usize| if a == b { 1 } else { 0 })),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug)]
struct VersionedPacket {
    version: usize,
    packet: Packet,
}

fn to_bytes(s: &str) -> BitVec<u8, Msb0> {
    let chars: Vec<_> = s.chars().collect();
    let mut result = bitvec![u8, Msb0;];
    for chunk in chars.chunks(2) {
        let i =
            ((chunk[0].to_digit(16).unwrap() as u8) << 4) + chunk[1].to_digit(16).unwrap() as u8;
        let mut tail: BitVec<u8, Msb0> = BitVec::from_element(i);
        result.append(&mut tail);
    }
    result
}

fn parse(bytes: &BitSlice<u8, Msb0>) -> (VersionedPacket, &BitSlice<u8, Msb0>) {
    let version = bytes[0..3].load_be();
    let id = bytes[3..6].load_be();
    let (packet, remaining) = if id == 4 {
        let mut buf = bitvec![u8, Msb0;];
        let mut i = 6;
        while bytes[i] {
            buf.extend_from_bitslice(&bytes[(i + 1)..(i + 5)]);
            i += 5;
        }
        buf.extend_from_bitslice(&bytes[(i + 1)..(i + 5)]);
        (Packet::Literal(buf.load_be()), &bytes[(i + 5)..])
    } else {
        if bytes[6] {
            let count = bytes[7..18].load_be();
            let mut bytes = &bytes[18..];
            let mut children = vec![];
            for _ in 0..count {
                let (child, remaining_slice) = parse(bytes);
                children.push(child);
                bytes = remaining_slice;
            }
            (Packet::Operator { id, children }, bytes)
        } else {
            let mut remaining: usize = bytes[7..22].load_be();
            let mut bytes = &bytes[22..];
            let mut children = vec![];
            while remaining > 0 {
                let (child, remaining_slice) = parse(bytes);
                children.push(child);
                remaining -= bytes.len() - remaining_slice.len();
                bytes = remaining_slice;
            }
            (Packet::Operator { id, children }, bytes)
        }
    };
    (VersionedPacket { version, packet }, remaining)
}

fn solve(s: &str) -> usize {
    let bytes = to_bytes(s);
    let (packet, _) = parse(&bytes);
    packet.packet.eval()
}

fn main() {
    let path = args().nth(1).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    println!("{}", solve(&buf.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let input = "C200B40A82";
        let expected = 3;
        assert_eq!(solve(input), expected);

        let input = "04005AC33890";
        let expected = 54;
        assert_eq!(solve(input), expected);

        let input = "880086C3E88112";
        let expected = 7;
        assert_eq!(solve(input), expected);

        let input = "CE00C43D881120";
        let expected = 9;
        assert_eq!(solve(input), expected);

        let input = "D8005AC2A8F0";
        let expected = 1;
        assert_eq!(solve(input), expected);

        let input = "F600BC2D8F";
        let expected = 0;
        assert_eq!(solve(input), expected);

        let input = "9C005AC2F8F0";
        let expected = 0;
        assert_eq!(solve(input), expected);

        let input = "9C0141080250320F1802104A08";
        let expected = 1;
        assert_eq!(solve(input), expected);
    }
}
