use std::env::args;
use std::fs::File;
use std::io::Read;

const MAX: u32 = 256;

fn main() {
    let path = args().nth(1).unwrap();
    let lengths = load_input(&path);

    let mut list: Vec<_> = (0..MAX).collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    for length in lengths {
        let mut i = current_position;

        let mut temp: Vec<_> = list.iter()
            .cloned()
            .cycle()
            .skip(current_position)
            .take(length)
            .collect();
        temp.reverse();

        for n in temp {
            list[i] = n;
            i = (i + 1) % list.len();
        }

        current_position = (current_position + length + skip_size) % list.len();
        skip_size += 1;
    }

    println!("Product of first two digits: {}", list[0] * list[1]);
}

fn load_input(path: &str) -> Vec<usize> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().split(',').map(|s| s.parse().unwrap()).collect()
}
