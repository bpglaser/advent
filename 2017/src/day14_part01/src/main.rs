use std::env::args;

use day10::knot_hash as hash;

fn main() {
    let input = args().nth(1).expect("valid input arg");
    let used_count: u32 = (0..128)
        .map(|n| format!("{}-{}", input, n))
        .map(|s| hash(s.as_bytes()))
        .flat_map(|v| v.into_iter())
        .map(|n| n.count_ones())
        .sum();
    println!("{:?}", used_count);
}

mod day10 {
    pub fn knot_hash(input: &[u8]) -> Vec<u8> {
        let mut input = Vec::from(input);
        input.extend_from_slice(&[17, 31, 73, 47, 23]);

        let mut sparse_hash: Vec<_> = (0..256u32).collect();
        let mut current_position = 0;
        let mut skip_size = 0;

        for _ in 0..64 {
            for length in &input {
                let length = *length as usize;
                let mut i = current_position;

                let mut temp: Vec<_> = sparse_hash
                    .iter()
                    .cloned()
                    .cycle()
                    .skip(current_position)
                    .take(length)
                    .collect();
                temp.reverse();

                for n in temp {
                    sparse_hash[i] = n;
                    i = (i + 1) % sparse_hash.len();
                }

                current_position = (current_position + length + skip_size) % sparse_hash.len();
                skip_size += 1;
            }
        }

        let mut dense_hash: Vec<u8> = vec![];
        for chunk in sparse_hash.chunks(16) {
            let mut n = chunk[0];
            for b in chunk.iter().skip(1) {
                n ^= b;
            }
            dense_hash.push(n as u8);
        }

        dense_hash
    }
}