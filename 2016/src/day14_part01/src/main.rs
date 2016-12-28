extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::env::args;

fn main() {
    let salt = get_salt();

    let mut pending_keys: Vec<Key> = vec![];

    let mut index: usize = 0;
    while pending_keys.iter().take_while(|k| k.finalization.is_some()).count() < 64 {
        let source = format!("{}{}", salt, index);
        let hash = md5_hash(&source);

        if let Some(c) = find_repeated_sequence(&hash, 5) {
            for key in pending_keys.iter_mut() {
                if key.repeating_char == c && index - key.discovery <= 1000 {
                    println!("Promoting: {:?}", key);
                    key.finalization = Some(index);
                    key.finalization_hash = Some(hash.to_owned());
                }
            }
        }

        if let Some(new_key) = Key::create(source, index, hash) {
            println!("New candidate:\t\t{:?}", new_key);
            pending_keys.push(new_key);
        }

        pending_keys.retain(|k| k.finalization.is_some() || index - k.discovery <= 1000);

        index += 1;
    }

    pending_keys.truncate(64);
    let last_key = pending_keys.last().unwrap();
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
