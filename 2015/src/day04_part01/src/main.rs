extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::env::args;

fn main() {
    let input = get_input();
    let mut i = 0;
    loop {
        let s = format!("{}{}", input, i);
        let hash = md5_hash(&s);
        if hash.chars().take_while(|c| c == &'0').count() == 5 {
            break;
        }
        i += 1;
    }
    println!("Lowest int: {}", i);
}

fn md5_hash(s: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(s);
    digest.result_str()
}

fn get_input() -> String {
    args().skip(1).next().expect("Invalid args")
}
