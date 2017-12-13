extern crate petgraph;

use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::Read;

use petgraph::prelude::*;

// 112 too low
// 1999 too high

fn main() {
    let path = args().nth(1).unwrap();
    let graph = load_input(&path);

    let mut count = 0;

    let root = graph
        .node_indices()
        .find(|idx| graph.node_weight(*idx) == Some(&0))
        .unwrap();

    for idx in graph.node_indices() {
        if idx == root {
            continue;
        }
        let mut dfs = Dfs::new(&graph, root);
        while let Some(nx) = dfs.next(&graph) {
            if graph.node_weight(nx) == Some(&0) {
                count += 1;
            }
        }
    }

    println!("answer: {:?}", count);
}

fn load_input(path: &str) -> UnGraph<u32, u32> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut graph = Graph::new_undirected();

    let mut temp_connections = HashMap::new();
    for (n, connections) in buf.lines().map(parse_line) {
        let idx = graph.add_node(n);
        temp_connections.insert(n, (idx, connections));
    }
    for (n, &(idx, ref connections)) in &temp_connections {
        for conn in connections {
            let (idx2, _) = temp_connections[&conn];
            graph.add_edge(idx, idx2, 0);
        }
    }

    graph
}

fn parse_line(s: &str) -> (u32, Vec<u32>) {
    let mut iter = s.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    iter.next();
    let connections = iter.map(strip_trailing_comma)
        .map(|s| s.parse().unwrap())
        .collect();
    (n, connections)
}

fn strip_trailing_comma(s: &str) -> &str {
    if s.ends_with(',') {
        &s[..s.len() - 1]
    } else {
        s
    }
}
