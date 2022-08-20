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

fn sum_versions(p: &VersionedPacket) -> usize {
    match &p.packet {
        Packet::Literal(_) => p.version,
        Packet::Operator { children, .. } => children.iter().map(sum_versions).sum(),
    }
}

fn solve(s: &str) -> usize {
    let bytes = to_bytes(s);
    let (packet, _) = parse(&bytes);
    sum_versions(&packet)
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
        let input = "D2FE28";
        let expected = 6;
        assert_eq!(solve(input), expected);

        let input = "8A004A801A8002F478";
        let expected = 16;
        assert_eq!(solve(input), expected);

        let input = "620080001611562C8802118E34";
        let expected = 12;
        assert_eq!(solve(input), expected);

        let input = "C0015000016115A2E0802F182340";
        let expected = 23;
        assert_eq!(solve(input), expected);

        let input = "A0016C880162017C3686B18A3D4780";
        let expected = 31;
        assert_eq!(solve(input), expected);
    }
}
