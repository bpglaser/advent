use std::io::stdin;

fn main() {
    let count = read_lines().iter().filter(|s| is_valid_triangle(&s)).count();
    println!("Valid triangle count: {}", count);
}

fn read_lines() -> Vec<String> {
    let mut lines = vec![];
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("error reading stdin");
        if buf.trim().is_empty() {
            break;
        } else {
            lines.push(buf);
        }
    }
    lines
}

fn is_valid_triangle(input: &str) -> bool {
    let mut sides: Vec<isize> = input.split_whitespace().map(|s| s.parse().expect("error parsing number")).collect();
    sides.sort();
    (sides[0] + sides[1]) > sides[2]
}
