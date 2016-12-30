use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

use Operation::*;

fn main() {
    let mut wires: HashMap<&str, Option<u16>> = HashMap::new();
    let mut gates: Vec<Gate> = load_lines().iter().map(|s| Gate::parse(&s)).collect();

    let mut to_remove = vec![];
    for gate in gates

    let mut wires: Vec<(&str, Option<u16>)> = wires.into_iter().collect();
    wires.sort_by_key(|&(k, _)| k);
    for (label, value) in wires {
        println!("[{}]\t=>\t{:?}", label, value);
    }
}

#[derive(PartialEq)]
enum Operation {
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOOP,
}

struct Gate {
    input_a: String,
    input_b: Option<String>,
    operation: Operation,
    output: String,
}

impl Gate {
    fn parse(line: &str) -> Gate {
        let words: Vec<&str> = line.split_whitespace().collect();

        let operation;
        let input_a;
        let mut input_b = None;
        let output = (*words.last().unwrap()).to_owned();

        if words[0] == "NOT" {
            operation = NOT;
            input_a = words[1].to_owned();
        } else {
            input_a = words[0].to_owned();

            operation = match words[1] {
                "AND" => AND,
                "OR" => OR,
                "LSHIFT" => LSHIFT,
                "RSHIFT" => RSHIFT,
                "->" => NOOP,
                _ => panic!("Invalid operator"),
            };

            if operation != NOOP {
                input_b = Some(words[2].to_owned());
            }

        }

        Gate { input_a: input_a, input_b: input_b, operation: operation, output: output }
    }
}

fn load_lines() -> Vec<String> {
    let mut file = File::open(args().nth(1).unwrap()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.to_owned()).collect()
}
