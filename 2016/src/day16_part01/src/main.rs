use std::env::args;

fn main() {
    let (mut state, disk_size) = get_params();
    println!("[0]\t\t{}", state);

    let mut count = 1;
    while state.len() < disk_size {
        dragonify(&mut state);
        println!("[{}]\t\t{}", count, state);
        count += 1;
    }

    state.truncate(disk_size);
    println!("[Truncated]\t{}", state);

    let sum = checksum(&state);
    println!("[Checksum]\t{}", sum);
}

fn dragonify(a: &mut String) {
    let b: String = a.chars().rev().map(swap_binary).collect();
    a.push('0');
    a.push_str(&b);
}

fn swap_binary(c: char) -> char {
    match c {
        '1' => '0',
        '0' => '1',
        _ => panic!("Invalid binary: {}", c),
    }
}

fn checksum(s: &str) -> String {
    let mut sum = String::new();
    let mut iter = s.chars();
    while let Some(a) = iter.next() {
        let b = iter.next().unwrap();
        if a == b {
            sum.push('1');
        } else {
            sum.push('0');
        }
    }
    if sum.len() % 2 == 0 {
        return checksum(&sum);
    }
    sum
}

fn get_params() -> (String, usize) {
    let args: Vec<String> = args().skip(1).collect();
    (args[0].to_owned(), args[1].parse().unwrap())
}
