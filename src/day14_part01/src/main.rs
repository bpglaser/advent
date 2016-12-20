extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::env::args;

fn main() {
    let salt = get_salt();

    let mut pending_keys: Vec<Key> = vec![];
    let mut finalized_key_indexes: Vec<usize> = vec![];

    let mut index: usize = 0;
    'outer: loop {
        let source = format!("{}{}", salt, index);
        let hash = md5_hash(&source);

        if let Some(c) = find_repeated_sequence(&hash, 5) {
            for (i, key) in pending_keys.iter_mut().enumerate() {
                if key.repeating_char == c && index - key.discovery <= 1000 {
                    println!("Promoting: {:?}", key);
                    key.finalization = Some(index);
                    key.finalization_hash = Some(hash.to_owned());
                    finalized_key_indexes.push(i);
                    if finalized_key_indexes.len() == 64 {
                        break 'outer;
                    }
                }
            }
        }

        if let Some(new_key) = Key::create(source, index, hash) {
            println!("New candidate:\t\t{:?}", new_key);
            pending_keys.push(new_key);
        }

        index += 1;
    }

    let finalized_keys: Vec<&Key> = finalized_key_indexes.iter().map(|i| &pending_keys[*i]).collect();
    let last_key = finalized_keys.last().unwrap();
    println!("Answer key: {:?}", last_key);
    println!("Answer: {}", last_key.discovery);
}

#[derive(Clone, Debug)]
struct Key {
    source: String,
    initial_hash: String,
    repeating_char: char,
    discovery: usize,
    finalization: Option<usize>,
    finalization_hash: Option<String>,
}

impl Key {
    fn create(source: String, index: usize, hash: String) -> Option<Key> {
        if let Some(c) = find_repeated_sequence(&hash, 3) {
            Some(Key {
                source: source,
                initial_hash: hash,
                repeating_char: c,
                discovery: index,
                finalization: None,
                finalization_hash: None,
            })
        } else {
            None
        }
    }
}

fn md5_hash(source: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(source);
    digest.result_str()
}

fn find_repeated_sequence(s: &str, count: usize) -> Option<char> {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..(chars.len() - count + 1) {
        let target = chars[i];
        let mut found_count = 1;
        for offset in (1)..(count) {
            if chars[i + offset] == target {
                found_count += 1;
            }
        }
        if found_count == count {
            return Some(target);
        }
    }
    None
}

fn get_salt() -> String {
    args().skip(1).next().expect("Invalid number of args")
}
