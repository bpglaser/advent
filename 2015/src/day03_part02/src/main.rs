use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut set = HashSet::new();
    let mut santas = [(0, 0); 2];
    let mut i = 0;
    set.insert(santas[0]);
    for c in get_input().trim().chars() {
        let mut point = santas[i];
        match c {
            '^' => point = (point.0, point.1 + 1),
            'v' => point = (point.0, point.1 - 1),
            '>' => point = (point.0 + 1, point.1),
            '<' => point = (point.0 - 1, point.1),
            _ => panic!("Invalid char")
        }
        santas[i] = point;
        set.insert(point);
        i += 1;
        i %= 2;
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
