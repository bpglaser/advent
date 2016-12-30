use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

use Operation::*;

fn main() {
    let mut stack: Vec<_> = load_lines().iter().map(|s| Gate::parse(&s)).collect();
    let mut wires = HashMap::new();

    while stack.len() > 0 {
        let gate = stack.remove(0);
        if !gate.evaluate(&mut wires) {
            stack.push(gate);
        }
    }

    let mut results: Vec<_> = wires.iter().collect();
    results.sort_by_key(|pair| pair.0);
    for &(label, value) in results.iter() {
        println!("[{}]\t=>\t{:?}", label, value);
    }

    println!("Answer: {}", wires.get("a").unwrap().unwrap());
}

#[derive(PartialEq)]
enum Operation {
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    INSERT,
}

struct Gate {
    input_a: String,
    input_b: Option<String>,
    operation: Operation,
    output: String,
}

impl Gate {
    fn parse(line: &str) -> Gate {
        let words: Vec<_> = line.split_whitespace().collect();

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
                "->" => INSERT,
                _ => panic!("Invalid operator"),
            };

            if operation != INSERT {
                input_b = Some(words[2].to_owned());
            }

        }

        Gate { input_a: input_a, input_b: input_b, operation: operation, output: output }
    }

    fn evaluate(&self, wires: &mut HashMap<String, Option<u16>>) -> bool {
        let input_a = get_value(&self.input_a, &wires);
        let input_b = match self.input_b {
            Some(ref s) => get_value(s, &wires),
            None => None,
        };

        let result = match self.operation {
            NOT => {
                match input_a {
                    Some(n) => Some(!n),
                    None => None,
                }
            }
            AND => {
                match (input_a, input_b) {
                    (Some(a), Some(b)) => Some(a & b),
                    _ => None,
                }
            }
            OR => {
                match (input_a, input_b) {
                    (Some(a), Some(b)) => Some(a | b),
                    _ => None,
                }
            }
            LSHIFT => {
                match (input_a, input_b) {
                    (Some(a), Some(b)) => Some(a << b),
                    _ => None,
                }
            }
            RSHIFT => {
                match (input_a, input_b) {
                    (Some(a), Some(b)) => Some(a >> b),
                    _ => None,
                }
            }
            INSERT => input_a,
        };

        match result {
            Some(n) => {
                let entry = wires.entry(self.output.to_owned()).or_insert(None);
                *entry = Some(n);
                true
            }
            None => false,
        }
    }
}

fn get_value(s: &str, wires: &HashMap<String, Option<u16>>) -> Option<u16> {
    match s.parse() {
        Ok(n) => Some(n),
        Err(_) => {
            match wires.get(s) {
                Some(&Some(n)) => Some(n),
                _ => None,
            }
        }
    }
}

fn load_lines() -> Vec<String> {
    let mut file = File::open(args().nth(1).unwrap()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.to_owned()).collect()
}
