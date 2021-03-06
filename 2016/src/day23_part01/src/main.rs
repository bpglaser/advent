use std::cell::RefCell;
use std::env::args;
use std::fs::File;
use std::io::Read;

use Instruction::*;

fn main() {
    let path = get_path();
    let mut instructions = load_instructions(&path);

    let mut execution_index: isize = 0;
    let mut register = [0; 4];

    register[0] = 7; // initial state

    while execution_index >= 0 && (execution_index as usize) < instructions.len() {
        let mut update = RefCell::new(None);
        {
            let instruction = instructions.get(execution_index as usize).unwrap();
            // print!("{:?} => ", instruction);
            execution_index += instruction.execute(&mut register, &mut update);
            // println!("{:?}", register);
        }
        let value = update.borrow();
        if let Some(value) = *value {
            let offset = value.get(&register);
            let i = execution_index + offset - 1;
            if i >= 0 && (i as usize) < instructions.len() {
                let i = i as usize;
                // print!("Toggling: {:?} at [{}] ", instructions[i], i);
                instructions[i] = instructions[i].toggle();
                // println!("to: {:?}", instructions[i]);
            }
        }
    }

    println!("{:?}", register);
}

#[derive(Clone, Copy,Debug)]
enum Value {
    Letter(usize),
    Number(isize),
}

impl Value {
    fn parse(s: &str) -> Value {
        if let Ok(n) = s.parse() {
            Value::Number(n)
        } else {
            let n = match s {
                "a" => 0,
                "b" => 1,
                "c" => 2,
                "d" => 3,
                _ => panic!("Invalid register")
            };
            Value::Letter(n)
        }
    }

    fn get(&self, register: &[isize; 4]) -> isize {
        match self {
            &Value::Letter(i) => register[i],
            &Value::Number(n) => n,
        }
    }

    fn get_value(&self) -> Option<usize> {
        match self {
            &Value::Letter(i) => Some(i),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let words: Vec<&str> = line.split_whitespace().collect();
        let x = Value::parse(words[1]);
        let mut y = None;
        if let Some(word) = words.get(2) {
            y = Some(Value::parse(word));
        }
        match words[0] {
            "cpy" => {
                Cpy(x, y.unwrap())
            }
            "inc" => {
                Inc(x)
            }
            "dec" => {
                Dec(x)
            }
            "jnz" => {
                Jnz(x, y.unwrap())
            }
            "tgl" => {
                Tgl(x)
            }
            _ => panic!(),
        }
    }

    fn execute(&self, register: &mut [isize; 4], update: &mut RefCell<Option<Value>>) -> isize {
        match self {
            &Cpy(ref x, ref y) => {
                if let Some(y) = y.get_value() {
                    let x = x.get(&register);
                    register[y] = x;
                }
            }
            &Inc(ref x) => {
                if let Some(x) = x.get_value() {
                    register[x] += 1;
                }
            }
            &Dec(ref x) => {
                if let Some(x) = x.get_value() {
                    register[x] -= 1;
                }
            }
            &Jnz(ref x, ref y) => {
                let x = x.get(&register);
                if x != 0 {
                    return y.get(&register);
                }
            }
            &Tgl(ref x) => {
                *update.borrow_mut() = Some(*x);
            }
        }
        1
    }

    fn toggle(&self) -> Instruction {
        match self {
            &Inc(x) => Dec(x),
            &Dec(x) => Inc(x),
            &Tgl(x) => Inc(x),
            &Jnz(x, y) => Cpy(x, y),
            &Cpy(x, y) => Jnz(x, y),
        }
    }
}

fn load_instructions(path: &str) -> Vec<Instruction> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(Instruction::from_line).collect()
}

fn get_path() -> String {
    args().skip(1).next().expect("Invalid input")
}
