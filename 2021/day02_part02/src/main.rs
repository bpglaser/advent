use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::panic;

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().skip(1).next().ok_or("not enough args")?;
    let content = read_to_string(&path)?;
    let (x, y, _) = content.lines().map(parse_line).fold((0, 0, 0), do_move);
    println!("{}", x * y);
    Ok(())
}

fn parse_line(line: &str) -> (&str, i32) {
    let i = line.find(' ').expect("no space in line");
    (&line[0..i], line[i + 1..].parse().expect("invalid number in line"))
}

fn do_move((x, y, a): (i32, i32, i32), (action, n): (&str, i32)) -> (i32, i32, i32) {
    match action {
        "forward" => (x + n, y + a * n, a),
        "down" => (x, y, a + n),
        "up" => (x, y, a - n),
        _ => panic!("unknown action: {}", action),
    }
}
