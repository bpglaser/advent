use std::env::args;
use std::fs::File;
use std::io::Read;

use DanceMove::*;

fn main() {
    let path = args().nth(1).expect("valid input path");
    let dance_moves = load_input(&path);
    let state = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];
    let dance = Dance { state, dance_moves };
    let result = dance.we_can_dance();
    println!("{}", result.into_iter().collect::<String>());
}

fn load_input(path: &str) -> Vec<DanceMove> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.split(',').map(DanceMove::from).collect()
}

#[derive(Debug)]
struct Dance<'a> {
    state: Vec<&'a str>,
    dance_moves: Vec<DanceMove>,
}

impl<'a> Dance<'a> {
    fn we_can_dance(mut self) -> Vec<&'a str> {
        for dm in self.dance_moves {
            dm.execute_on(&mut self.state);
        }

        self.state
    }
}

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(String, String),
}

impl DanceMove {
    fn execute_on(&self, state: &mut Vec<&str>) {
        match self {
            &Spin(n) => {
                for _ in 0..n {
                    let dm = state.pop().unwrap();
                    state.insert(0, dm);
                }
            }
            &Exchange(n, m) => state.swap(n, m),
            &Partner(ref a, ref b) => {
                let n = state.iter().position(|s| s == a).unwrap();
                let m = state.iter().position(|s| s == b).unwrap();
                state.swap(n, m);
            }
        }
    }
}

impl<'a> From<&'a str> for DanceMove {
    fn from(s: &str) -> Self {
        let (prefix, remainder) = s.split_at(1);
        match prefix {
            "s" => {
                let n = remainder.parse().expect("Valid number");
                Spin(n)
            }
            "x" => {
                let (left, right) = split_slash(remainder);
                Exchange(left.parse().unwrap(), right.parse().unwrap())
            }
            "p" => {
                let (left, right) = split_slash(remainder);
                Partner(left.to_owned(), right.to_owned())
            }
            _ => panic!("Invalid dance move: {}", s),
        }
    }
}

fn split_slash(s: &str) -> (&str, &str) {
    let mut words = s.split('/');
    let left = words.next().unwrap();
    let right = words.next().unwrap();
    (left, right)
}
