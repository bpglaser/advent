use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut set = HashSet::new();
    let mut point = (0, 0);
    set.insert(point);
    for c in get_input().trim().chars() {
        match c {
            '^' => point = (point.0, point.1 + 1),
            'v' => point = (point.0, point.1 - 1),
            '>' => point = (point.0 + 1, point.1),
            '<' => point = (point.0 - 1, point.1),
            _ => panic!("Invalid char")
        }
        set.insert(point);
    }
    println!("Unique houses: {}", set.len());
}

fn get_input() -> String {
    let path = args().skip(1).next().expect("Invalid args");
    let mut file = File::open(path).expect("Error reading file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");
    buf
}
