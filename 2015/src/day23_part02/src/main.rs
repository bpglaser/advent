use std::env::args;
use std::fs::File;
use std::io::{Error, Read};

use Instruction::*;

const REG_SIZE: usize = 3;
type Register = [isize; REG_SIZE];

fn main() {
    let path = get_path().expect("Invalid input");
    let lines = load_lines(&path).expect("Error reading file");
    let instructions: Vec<Instruction> = lines.into_iter().map(|s| Instruction::parse(&s)).collect();

    let mut register: Register = [0; REG_SIZE];
    register[0] = 1; // Initial value as per part 2 instructions

    let mut execution_index: isize = 0;

    loop {
        let mut execution_offset = 1;

        match instructions[execution_index as usize] {
            HLF(ref value) => *value.get(&mut register) /= 2,
            TPL(ref value) => *value.get(&mut register) *= 3,
            INC(ref value) => *value.get(&mut register) += 1,
            JMP(ref offset) => execution_offset = *offset.get(&mut register),
            JIE(ref value, ref offset) => {
                if *value.get(&mut register) % 2 == 0 {
                    execution_offset = *offset.get(&mut register);
                }
            }
            JIO(ref value, ref offset) => {
                if *value.get(&mut register) == 1 {
                    execution_offset = *offset.get(&mut register);
                }
            }
        }
        
        let new_execution_index = execution_index + execution_offset;
        if new_execution_index < 0 || new_execution_index >= instructions.len() as isize {
            break;
        }
        execution_index = new_execution_index;
    }

    println!("A: {}", register[0]);
    println!("B: {}", register[1]);
}

enum Value {
    A,
    B,
    Number(isize),
}

impl Value {
    fn parse(s: &str) -> Self {
        match s {
            "a" => Value::A,
            "b" => Value::B,
            _ => match s.parse() {
                Ok(n) => Value::Number(n),
                Err(_) => panic!("NaN"),
            }
        }
    }

    fn get<'a>(&self, register: &'a mut Register) -> &'a mut isize {
        match self {
            &Value::A => &mut register[0],
            &Value::B => &mut register[1],
            &Value::Number(n) => {
                register[2] = n;
                &mut register[2]
            }
        }
    }
}

enum Instruction {
    HLF(Value),
    TPL(Value),
    INC(Value),
    JMP(Value),
    JIE(Value, Value),
    JIO(Value, Value),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let words: Vec<&str> = s.split_whitespace().map(|s| s.trim_right_matches(',')).collect();
        let first = Value::parse(words[1]);
        let second = words.get(2).map(|s| Value::parse(s));

        match words[0] {
            "hlf" => HLF(first),
            "tpl" => TPL(first),
            "inc" => INC(first),
            "jmp" => JMP(first),
            "jie" => JIE(first, second.expect("Invalid instruction args")),
            "jio" => JIO(first, second.expect("Invalid instruction args")),
            _ => panic!("Invalid instruction"),
        }
    }
}

fn get_path() -> Option<String> {
    args().skip(1).next()
}

fn load_lines(path: &str) -> Result<Vec<String>, Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf.trim().lines().map(|s| s.to_owned()).collect())
}