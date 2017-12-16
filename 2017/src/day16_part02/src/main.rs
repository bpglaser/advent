use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use DanceMove::*;
use Program::*;

const REPITITION: usize = 1_000_000_000;

fn main() {
    let path = args().nth(1).expect("valid input path");

    let dance_moves = load_input(&path);
    let state = vec![A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P];
    let mut dance = Dance { state, dance_moves };

    let mut cache = vec![];

    for i in 0..REPITITION {
        if cache.contains(&dance.state) {
            let state = cache[REPITITION % i].clone();
            println!("{}", state.into_iter().map(char::from).collect::<String>());
            return;
        }
        cache.push(dance.state.clone());
        dance.we_can_dance();
    }
}

fn load_input(path: &str) -> Vec<DanceMove> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.split(',').map(DanceMove::from).collect()
}

#[derive(Clone, Eq, Debug, Hash, PartialEq)]
enum Program {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => A,
            "b" => B,
            "c" => C,
            "d" => D,
            "e" => E,
            "f" => F,
            "g" => G,
            "h" => H,
            "i" => I,
            "j" => J,
            "k" => K,
            "l" => L,
            "m" => M,
            "n" => N,
            "o" => O,
            "p" => P,
            _ => panic!("invalid program {}", s),
        })
    }
}

impl From<Program> for char {
    fn from(program: Program) -> Self {
        match program {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
            I => 'i',
            J => 'j',
            K => 'k',
            L => 'l',
            M => 'm',
            N => 'n',
            O => 'o',
            P => 'p',
        }
    }
}

#[derive(Debug)]
struct Dance {
    state: Vec<Program>,
    dance_moves: Vec<DanceMove>,
}

impl Dance {
    fn we_can_dance(&mut self) {
        for dm in &self.dance_moves {
            dm.execute_on(&mut self.state);
        }
    }
}

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(Program, Program),
}

impl DanceMove {
    fn execute_on(&self, state: &mut Vec<Program>) {
        match *self {
            Spin(n) => {
                for _ in 0..n {
                    let dm = state.pop().unwrap();
                    state.insert(0, dm);
                }
            }
            Exchange(n, m) => state.swap(n, m),
            Partner(ref a, ref b) => {
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
                Partner(left.parse().unwrap(), right.parse().unwrap())
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
