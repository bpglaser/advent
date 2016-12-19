extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::env::args;

fn main() {
    let salt = get_salt();

    let mut confirmed_keys: Vec<Key> = vec![];
    let mut pending_keys: Vec<Key> = vec![];

    let mut index: usize = 0;
    while confirmed_keys.len() < 64 {
        let source = format!("{}{}", salt, index);
        let hash = hash(&source);

        if let Some(c) = find_quadruple(&hash) {
            let mut confirmed_indexes: Vec<usize> = vec![];

            for (i, key) in pending_keys.iter().enumerate() {
                if key.repeating_char == c {
                    confirmed_indexes.push(i);
                }
            }

            for i in confirmed_indexes {
                let mut key = pending_keys.remove(i);
                key.finalization = Some(index);
                confirmed_keys.push(key);
            }
        }

        if let Some(new_key) = Key::create(source, index, hash) {
            pending_keys.push(new_key);
        }

        index += 1;
    }
}

struct Key {
    source: String,
    hash: String,
    repeating_char: char,
    discovery: usize,
    finalization: Option<usize>,
}

impl Key {
    fn create(source: String, index: usize, hash: String) -> Option<Key> {
        if let Some(c) = find_triple(&hash) {
            Some(Key {
                source: source,
                hash: hash,
                repeating_char: c,
                discovery: index,
                finalization: None,
            })
        } else {
            None
        }
    }
}

fn hash(source: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(source);
    digest.result_str()
}

fn find_triple(s: &str) -> Option<char> {
    unimplemented!()
}

fn find_quadruple(s: &str) -> Option<char> {
    unimplemented!()
}

fn get_salt() -> String {
    args().skip(1).next().expect("Invalid number of args")
}
