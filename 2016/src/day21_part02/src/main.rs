use std::env::args;
use std::fs::File;
use std::io::Read;

use Instruction::*;

fn main() {
    let (password, path) = get_args();
    let mut password_chars = password.chars().collect();
    let instructions = load_instructions(&path);
    for instruction in instructions.iter().rev() {
        instruction.inverse().execute(&mut password_chars);
    }
    let answer: String = password_chars.into_iter().collect();
    println!("scrabled password: {}", answer);
}

#[derive(Clone, Debug)]
enum Instruction {
    SwapPosition { x: usize, y: usize },
    SwapLetter { x: char, y: char },
    RotateSteps { count: isize },
    RotateLetter { letter: char },
    ReverseRange { x: usize, y: usize },
    MoveLetter { x: usize, y: usize },
    InverseRotateLetter { letter: char },
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "swap" => {
                if words[1] == "position" {
                    let x = words[2].parse().unwrap();
                    let y = words[5].parse().unwrap();
                    return SwapPosition { x: x, y: y };
                } else {
                    let x = words[2].chars().next().unwrap();
                    let y = words[5].chars().next().unwrap();
                    return SwapLetter { x: x, y: y };
                }
            }
            "rotate" => {
                if words[1] == "based" {
                    let letter = words[6].chars().next().unwrap();
                    return RotateLetter { letter: letter };
                } else {
                    let mut n = words[2].parse().unwrap();
                    if words[1] == "left" {
                        n *= -1;
                    }
                    return RotateSteps { count: n };
                }
            }
            "reverse" => {
                let x = words[2].parse().unwrap();
                let y = words[4].parse().unwrap();
                return ReverseRange { x: x, y: y };
            }
            "move" => {
                let x = words[2].parse().unwrap();
                let y = words[5].parse().unwrap();
                return MoveLetter { x: x, y: y };
            }
            _ => panic!(),
        }
    }

    fn inverse(&self) -> Instruction {
        match self {
            &SwapPosition { .. } => self.clone(),
            &SwapLetter { x, y } => SwapLetter { x: y, y: x },
            &RotateSteps { count } => RotateSteps { count: count * -1 },
            &RotateLetter { letter } => InverseRotateLetter { letter: letter },
            &ReverseRange { .. } => self.clone(),
            &MoveLetter { x, y } => MoveLetter { x: y, y: x },
            &InverseRotateLetter { letter } => RotateLetter { letter: letter },
        }
    }

    fn execute(&self, mut password: &mut Vec<char>) {
        match self {
            &SwapPosition { x, y } => password.swap(x, y),
            &SwapLetter { x, y } => {
                let pos_x = index_of(password, x);
                let pos_y = index_of(password, y);
                password.swap(pos_x, pos_y);
            }
            &RotateSteps { count } => {
                if count > 0 {
                    for _ in 0..count {
                        rotate_right(&mut password);
                    }
                } else {
                    for _ in 0..count.abs() {
                        rotate_left(&mut password);
                    }
                }
            }
            &RotateLetter { letter } => {
                let mut i = index_of(password, letter);
                if i >= 4 {
                    i += 1;
                }
                i += 1;
                for _ in 0..i {
                    rotate_right(&mut password);
                }
            }
            &ReverseRange { mut x, mut y } => {
                if x > y {
                    let a = x;
                    x = y;
                    y = a;
                }
                let buf: Vec<char> = password.drain(x..(y + 1)).collect();
                for c in buf {
                    password.insert(x, c);
                }
            }
            &MoveLetter { x, y } => {
                let c = password.remove(x);
                password.insert(y, c);
            }
            &InverseRotateLetter { letter } => {
                let count = match index_of(password, letter) {
                    0 | 1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 0,
                    7 => 4,
                    _ => panic!(),
                };
                for _ in 0..count {
                    rotate_left(&mut password);
                }
            }
        }
    }
}

fn rotate_left(v: &mut Vec<char>) {
    let c = v.remove(0);
    v.push(c);
}

fn rotate_right(v: &mut Vec<char>) {
    let c = v.pop().unwrap();
    v.insert(0, c);
}

fn index_of(word: &Vec<char>, c: char) -> usize {
    let mut index = 0;
    for letter in word {
        if letter == &c {
            return index;
        }
        index += 1;
    }
    panic!("Unable to find: {}", c)
}

fn load_instructions(path: &str) -> Vec<Instruction> {
    let mut file = File::open(path).expect(&format!("Invalid path: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect(&format!("Unable to read: {}", path));
    buf.lines().map(Instruction::from_line).collect()
}

fn get_args() -> (String, String) {
    let mut args = args().skip(1);
    (args.next().unwrap(), args.next().unwrap())
}
