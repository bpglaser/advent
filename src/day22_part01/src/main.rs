use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = get_path();
    let nodes = load_nodes(&path);
    let mut set: Vec<Link> = vec![];
    for a in nodes.iter() {
        for b in nodes.iter() {
            if a == b {
                continue;
            }
            if a.used > 0 && a.used <= b.avail {
                let link = Link::new(*a, *b);
                if !set.contains(&link) {
                    set.push(link);
                }
            }
        }
    }

    println!("Number of pairs: {}", set.len());
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

impl Node {
    fn from_line(line: &str) -> Node {
        let words: Vec<&str> = line.split_whitespace().map(|s| s.trim()).collect();
        let filesystem = words[0];
        let (x, y) = parse_coords(&filesystem);
        Node {
            x: x,
            y: y,
            size: parse_volume(words[1]),
            used: parse_volume(words[2]),
            avail: parse_volume(words[3]),
        }
    }
}

struct Link {
    a: Node,
    b: Node,
}

impl Link {
    fn new(a: Node, b: Node) -> Link {
        Link { a: a, b: b }
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        if self.a == other.a && self.b == other.b {
            return true;
        }
        if self.a == other.b && self.b == other.a {
            return true;
        }
        false
    }
}

fn parse_coords(s: &str) -> (usize, usize) {
    let mut x_encountered = false;
    let mut x = String::new();
    let mut y_encountered = false;
    let mut y = String::new();
    for c in s.chars() {
        if c == 'x' {
            x_encountered = true;
        } else if c == 'y' {
            y_encountered = true
        } else if c.is_digit(10) {
            if y_encountered {
                y.push(c);
            } else if x_encountered {
                x.push(c);
            } else {
                panic!();
            }
        }
    }
    (x.parse().expect("Error parsing coords"), y.parse().expect("Error parsing coords"))
}

fn parse_volume(s: &str) -> usize {
    s.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse().expect(&format!("Error parsing: {}", s))
}

fn load_nodes(path: &str) -> Vec<Node> {
    let mut file = File::open(path).expect(&format!("Error opening: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect(&format!("Error reading from: {}", path));
    buf.lines().skip(2).map(Node::from_line).collect()
}

fn get_path() -> String {
    args().skip(1).next().expect("Invalid input")
}
