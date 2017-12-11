use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).unwrap();
    let input = load_input(&path);

    let mut depth = 0;
    let mut total = 0;
    let mut garbage_mode = false;
    let mut skip_mode = false;

    for c in input.chars() {
        if garbage_mode {
            if skip_mode {
                skip_mode = false;
            } else {
                match c {
                    '!' => skip_mode = true,
                    '>' => garbage_mode = false,
                    _ => {},
                }
            }
        } else {
            match c {
                '{' => depth += 1,
                '}' => {
                    total += depth;
                    depth -= 1;
                }
                '<' => garbage_mode = true,
                _ => {},
            }
        }
    }

    println!("answer: {}", total);
}

fn load_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
