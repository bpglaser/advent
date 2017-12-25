use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).unwrap();
    let input = load_input(&path);
    let answer = do_puzzle(&input);
    println!("answer: {}", answer);
}

fn load_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

fn do_puzzle(input: &str) -> usize {
    let nodes = parse_nodes(input);

    let solver = Solver { nodes: &nodes };
    solver.walk(&HashSet::new(), 0).1
}

fn parse_nodes(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut words = line.split('/').map(|s| s.parse::<usize>().unwrap());
            (words.next().unwrap(), words.next().unwrap())
        })
        .collect()
}

struct Solver<'a> {
    nodes: &'a [(usize, usize)],
}

impl<'a> Solver<'a> {
    fn walk(&self, used_pieces: &HashSet<usize>, previous: usize) -> (usize, usize) {
        if self.nodes.len() == used_pieces.len() {
            return (0, 0);
        }

        let mut path = vec![];
        let mut used_pieces = used_pieces.clone();

        for (i, &(left, right)) in self.nodes.iter().enumerate() {
            if (left == previous || right == previous) && !used_pieces.contains(&i) {
                used_pieces.insert(i);
                let (longest_length, strength) = self.walk(&used_pieces, left + right - previous);
                used_pieces.remove(&i);
                path.push((longest_length + 1, strength + left + right));
            }
        }

        path.into_iter().max().unwrap_or((0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        let answer = do_puzzle(input);
        assert_eq!(19, answer);
    }
}
