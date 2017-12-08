use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).unwrap();
    let tree = load_input(&path);

    let unbalanced_name = tree.find_unbalanced_node();
    let answer_child = tree.find_black_sheep(unbalanced_name).unwrap();

    let other = tree.nodes[unbalanced_name]
        .children
        .iter()
        .find(|s| s.as_str() != answer_child)
        .unwrap();
    let diff = tree.total_weight(other) - tree.total_weight(answer_child);
    println!("answer: {}", tree.nodes[answer_child].weight + diff);
}

fn load_input(path: &str) -> Tree {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for node in buf.lines().map(|s| s.parse::<Node>().unwrap()) {
        nodes.insert(node.name.clone(), node);
    }

    let root = {
        let mut parent_havers: HashSet<&str> = HashSet::new();
        for node in nodes.values() {
            for child in node.children.iter() {
                if !parent_havers.contains(child.as_str()) {
                    parent_havers.insert(child);
                }
            }
        }

        nodes
            .keys()
            .find(|name| !parent_havers.contains(name.as_str()))
            .unwrap()
            .clone()
    };

    let tree = Tree {
        root: root.clone(),
        nodes,
    };
    tree.calculate_child_weight(&root);
    tree
}

#[derive(Debug)]
struct Tree {
    root: String,
    nodes: HashMap<String, Node>,
}

impl Tree {
    fn total_weight(&self, name: &str) -> i32 {
        if self.nodes[name].total_child_weight.get().is_none() {
            self.calculate_child_weight(name);
        }
        self.nodes[name].weight + self.nodes[name].total_child_weight.get().unwrap()
    }

    fn calculate_child_weight(&self, name: &str) {
        if self.nodes[name].children.len() == 0 {
            self.nodes[name].total_child_weight.set(Some(0));
            return;
        }

        for child_name in &self.nodes[name].children {
            let old = self.nodes[name].total_child_weight.get().unwrap_or(0);
            self.nodes[name].total_child_weight.set(Some(
                old +
                    self.total_weight(
                        child_name,
                    ),
            ));
        }
    }

    fn is_balanced(&self, name: &str) -> bool {
        if self.nodes[name].children.len() == 0 {
            return true;
        }

        let mut iterator = self.nodes[name].children.iter();

        let n = self.total_weight(iterator.next().unwrap());

        for name in iterator {
            if self.total_weight(name) != n {
                return false;
            }
        }
        true
    }

    fn find_unbalanced_node(&self) -> &str {
        let mut unbalanced_nodes = vec![];
        let mut queue = vec![(&self.root, 0)];

        while let Some((name, depth)) = queue.pop() {
            if !self.is_balanced(name) {
                unbalanced_nodes.push((name, depth));
            }
            for child_name in &self.nodes[name].children {
                queue.push((child_name, depth + 1));
            }
        }

        unbalanced_nodes.sort_by_key(|&(_, depth)| depth);
        unbalanced_nodes.pop().unwrap().0
    }

    fn find_black_sheep(&self, name: &str) -> Option<&str> {
        let mut counter: HashMap<i32, u32> = HashMap::new();
        for child_name in &self.nodes[name].children {
            let total_weight = self.total_weight(child_name);
            if !counter.contains_key(&total_weight) {
                counter.insert(total_weight, 0);
            }
            *counter.get_mut(&total_weight).unwrap() += 1;
        }

        let black_sheep_value = counter.into_iter().min_by_key(|&(_, count)| count).map(
            |(value, _)| value,
        );

        match black_sheep_value {
            None => None,
            Some(total_weight) => {
                self.nodes[name]
                    .children
                    .iter()
                    .find(|child| self.total_weight(child) == total_weight)
                    .map(|s| s.as_str())
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: i32,
    children: Vec<String>,
    total_child_weight: Cell<Option<i32>>,
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
            total_child_weight: Cell::new(None),
        })
    }
}

fn remove_parens(s: &str) -> String {
    s.chars().skip(1).take_while(|c| c != &')').collect()
}

fn remove_trailing_comma(s: &str) -> String {
    s.chars().take_while(|c| c != &',').collect()
}
