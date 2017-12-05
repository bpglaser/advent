use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).unwrap();
    let mut instructions = load_input(&path);

    let mut i = 0i32;
    let mut steps = 0;

    while i >= 0 && (i as usize) < instructions.len() {
        let instruction = &mut instructions[i as usize];
        i += *instruction;
        *instruction += 1;
        steps += 1;
    }
    println!("steps: {}", steps);
}

fn load_input(path: &str) -> Vec<i32> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.parse().unwrap()).collect()
}
