extern crate petgraph;

use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

use petgraph::prelude::*;

// 112 too low
// 1999 too high

fn main() {
    let path = args().nth(1).unwrap();
    let graph = load_input(&path);

    println!("{:?}", graph);

    // Starts from one because root always connects to root.
    let mut count = 1;

    let root = graph
        .node_indices()
        .find(|idx| graph.node_weight(*idx) == Some(&0))
        .unwrap();

    for idx in graph.node_indices() {
        if idx == root {
            println!("skipping: {:?}", idx);
            continue;
        }
        println!("finding path for: {:?}", idx);
        let mut dfs = Dfs::new(&graph, root);
        while let Some(nx) = dfs.next(&graph) {
            if nx == idx {
                count += 1;
                println!("{:?} connects to 0", idx);
            }
        }
    }

    println!("answer: {:?}", count);
}

fn load_input(path: &str) -> Graph<u32, u32> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut graph = Graph::new();

    let mut temp_connections = HashMap::new();
    for (n, connections) in buf.lines().map(parse_line) {
        let idx = graph.add_node(n);
        temp_connections.insert(n, (idx, connections));
    }
    for (_, &(idx1, ref connections)) in &temp_connections {
        for conn in connections {
            let (idx2, _) = temp_connections[&conn];
            graph.add_edge(idx1, idx2, 1);
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
