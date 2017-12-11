use std::env::args;
use std::fs::File;
use std::io::Read;

const MAX: u32 = 256;
static BYTE_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

fn main() {
    let path = args().nth(1).unwrap();
    let lengths = load_input(&path);
    // println!("input: {:?}", lengths);

    let mut sparse_hash: Vec<_> = (0..MAX).collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in &lengths {
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

    let mut dense_hash = vec![];
    for chunk in sparse_hash.chunks(16) {
        let mut n = chunk[0];
        for b in chunk.iter().skip(1) {
            n ^= b;
        }
        dense_hash.push(n);
    }

    for byte in dense_hash {
        print!("{:02x}", byte);
    }
    println!();
}

fn load_input(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut buf: Vec<_> = buf.trim().chars().map(|b| b as u8).collect();
    buf.extend_from_slice(&BYTE_SUFFIX);
    buf
}
