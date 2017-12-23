use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use Instruction::*;
use Value::*;

const LETTERS: &str = "abcdefgh";

type Register = Vec<i64>;

fn main() {
    let input_path = args().nth(1).expect("valid input path");
    let instructions = load_instructions(&input_path);

    println!("Loaded instructions:");
    for instruction in &instructions {
        println!("\t{:?}", instruction);
    }

    let mut machine = Machine::new(0, instructions);
    machine.execute();

    println!("The value left in register 'h' is {}", machine.register[LETTERS.chars().position(|c| c == 'h').unwrap()]);
}

fn load_instructions(path: &str) -> Vec<Instruction> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().lines().map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Machine {
    id: i64,
    execution_index: isize,
    instructions: Vec<Instruction>,
    register: Register,
}

impl Machine {
    fn new(id: i64, instructions: Vec<Instruction>) -> Self {
        let mut register = vec![0; LETTERS.len()];
        register[0] = 1; // set register a to 1
        Self {
            id,
            execution_index: 0,
            instructions,
            register,
        }
    }

    fn execute(&mut self) {
        loop {
            if self.execution_index == self.instructions.len() as isize {
                return;
            }
            if self.execution_index < 0 || self.execution_index as usize > self.instructions.len() {
                panic!("Jumped out of bounds at: {}", self.execution_index);
            }

            let instruction = self.instructions[self.execution_index as usize];
            instruction.execute(self);
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    SET(Value, Value),
    SUB(Value, Value),
    MUL(Value, Value),
    JNZ(Value, Value),
}

impl Instruction {
    fn execute(&self, machine: &mut Machine) {
        match *self {
            SET(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&machine.register);
                machine.register[x] = y;
            }
            SUB(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&machine.register);
                machine.register[x] -= y;
            }
            MUL(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&machine.register);
                machine.register[x] *= y;
            }
            JNZ(ref x, ref y) => {
                let x = x.resolve(&machine.register);
                let y = y.resolve(&machine.register) as isize;
                if x != 0 {
                    machine.execution_index += y;
                    return;
                }
            }
        }
        machine.execution_index += 1;
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        Ok(match iter.next() {
            Some("set") => {
                let (x, y) = get_double_value(iter);
                SET(x, y)
            }
            Some("sub") => {
                let (x, y) = get_double_value(iter);
                SUB(x, y)
            }
            Some("mul") => {
                let (x, y) = get_double_value(iter);
                MUL(x, y)
            }
            Some("jnz") => {
                let (x, y) = get_double_value(iter);
                JNZ(x, y)
            }
            _ => panic!("invalid instruction: {}", s),
        })
    }
}

fn get_double_value<'a, T: Iterator<Item = &'a str>>(mut iter: T) -> (Value, Value) {
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().parse().unwrap();
    (x, y)
}

#[derive(Copy, Clone, Debug)]
enum Value {
    Register(usize),
    Raw(i64),
}

impl Value {
    fn get(&self) -> usize {
        match *self {
            Register(i) => i,
            Raw(_) => panic!("tried to get a raw"),
        }
    }

    fn resolve(&self, register: &[i64]) -> i64 {
        match *self {
            Register(i) => register[i],
            Raw(n) => n,
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse() {
            Ok(n) => Raw(n),
            Err(_) => {
                assert!(s.len() == 1);
                let letter = s.chars().next().unwrap();
                Register(LETTERS.chars().position(|c| c == letter).unwrap())
            }
        })
    }
}
