extern crate serde_json;

use serde_json::Value;
use serde_json::Value::*;

use std::env::args;
use std::fs::File;

fn main() {
    let mut stack: Vec<Value> = vec![load_json()];
    let mut total = 0;

    while stack.len() > 0 {
        let value = stack.remove(0);

        match value {
            I64(n) => total += n,
            U64(n) => total += n as i64,
            F64(_) => unimplemented!(),
            Array(mut children) => stack.append(&mut children),
            Object(map) => stack.extend(map.into_iter().map(|pair| pair.1)),
            _ => {},
        }
    }

    println!("Total: {}", total);
}

fn load_json() -> Value {
    let path = args().nth(1).expect("Invalid args");
    let file = File::open(path).expect("Error opening file");
    serde_json::from_reader(file).expect("Error decoding json")
}
