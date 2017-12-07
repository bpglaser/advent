use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).unwrap();
    let nodes = load_input(&path);

    let mut parent_map: HashMap<String, u32> = HashMap::new();
    for node in &nodes {
        for child in &node.children {
            if !parent_map.contains_key(child) {
                parent_map.insert(child.clone(), 0);
            }
            *parent_map.get_mut(child).unwrap() += 1;
        }
    }

    let answer = nodes.iter()
        .find(|node| !parent_map.contains_key(&node.name))
        .unwrap();
    println!("Root node: {:?}", answer);
}

fn load_input(path: &str) -> Vec<Node> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
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
