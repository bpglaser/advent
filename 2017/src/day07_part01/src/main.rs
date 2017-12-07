use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).unwrap();
    let mut nodes = load_input(&path);
}

fn load_input(path: &str) -> Node {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut orphans = vec![];
    for line in buf.lines().map(|s| s.parse().unwrap()) {
    }
}

struct Node {
    name: String,
    weight: u32,
    children: Vec<IncompleteNode>,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let words: Vec<_> = s.split_whitespace().collect();

        let name = words[0].to_owned();

        let weight = remove_parens(words[1]).parse().unwrap();

        let children = words
            .into_iter()
            .skip(3)
            .map(remove_trailing_comma)
            .map(|s| IncompleteNode::Placeholder(s))
            .collect();

        Ok(Node {
            name,
            weight,
            children,
        })
    }
}

fn remove_parens(s: &str) -> String {
    s.chars().skip(1).take_while(|c| c != &')').collect()
}

fn remove_trailing_comma(s: &str) -> String {
    s.chars().take_while(|c| c != &',').collect()
}

enum IncompleteNode {
    Placeholder(String),
    Complete(Node),
}
