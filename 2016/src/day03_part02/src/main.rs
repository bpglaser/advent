use std::io::stdin;

fn main() {
    let triangles = read_lines().iter().map(|s| parse_triangle(&s)).collect();
    let new_triangles = rotate(triangles);
    let count = new_triangles.iter().filter(|t| is_valid_triangle(t.0, t.1, t.2)).count();
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

fn parse_triangle(s: &str) -> (usize, usize, usize) {
    let buf: Vec<usize> = s.split_whitespace().map(|s| s.parse().expect("error parsing number")).collect();
    (buf[0], buf[1], buf[2])
}

fn rotate(mut triangles: Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut new_triangles = vec![];
    while !triangles.is_empty() {
        let a = triangles.remove(0);
        let b = triangles.remove(0);
        let c = triangles.remove(0);
        new_triangles.push((a.0, b.0, c.0));
        new_triangles.push((a.1, b.1, c.1));
        new_triangles.push((a.2, b.2, c.2));
    }
    new_triangles
}

fn is_valid_triangle(a: usize, b: usize, c: usize) -> bool {
    let mut sides = vec![a, b, c];
    sides.sort();
    (sides[0] + sides[1]) > sides[2]
}
