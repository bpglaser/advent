use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).unwrap();
    let mut values = load_input(&path);

    let mut seen = HashMap::new();

    let mut count = 0;
    loop {
        redistribute(&mut values);
        count += 1;

        if seen.contains_key(&values) {
            let prev = seen.get(&values).unwrap();
            println!("count: {}", count - prev);
            break;
        } else {
            seen.insert(values.clone(), count);
        }
    }
}

fn redistribute(values: &mut Vec<i32>) {
    let len = values.len();
    let (mut i, mut max) = find_max(&values);
    values[i] = 0;
    while max > 0 {
        i += 1;
        values[i % len] += 1;
        max -= 1;
    }
}

// Iterator::max_by_key finds the final max value in the collection
fn find_max(values: &[i32]) -> (usize, i32) {
    let mut i = 0;
    let mut max = i32::min_value();
    for (j, n) in values.iter().enumerate() {
        if n > &max {
            i = j;
            max = *n;
        }
    }
    (i, max)
}

fn load_input(path: &str) -> Vec<i32> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
