use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process;
use std::str::FromStr;

use Instruction::*;
use Value::*;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

type Register = Vec<i64>;
type SoundStack = Vec<i64>;

fn main() {
    let input_path = args().nth(1).expect("valid input path");
    let instructions = load_instructions(&input_path);
    println!("Loaded instructions:");
    for instruction in &instructions {
        println!("\t{:?}", instruction);
    }

    let mut machine = Machine::new(instructions);
    machine.execute();
}

fn load_instructions(path: &str) -> Vec<Instruction> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().lines().map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Machine {
    execution_index: isize,
    instructions: Vec<Instruction>,
    register: Register,
    sound_stack: SoundStack,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { execution_index: 0, instructions, register: vec![0; LETTERS.len()], sound_stack: vec![] }
    }

    fn execute(&mut self) {
        loop {
            println!("STATE:\n\tInd: {}\n\tReg: {:?}\n\tSnd: {:?}", self.execution_index, self.register, self.sound_stack);
            if self.execution_index < 0 || self.execution_index as usize >= self.instructions.len() {
                panic!("Jumped out of bounds!");
            }

            self.instructions[self.execution_index as usize].execute(&mut self.execution_index, &mut self.register, &mut self.sound_stack);
        }
    }
}

#[derive(Debug)]
enum Instruction {
    SND(Value),
    SET(Value, Value),
    ADD(Value, Value),
    MUL(Value, Value),
    MOD(Value, Value),
    RCV(Value),
    JGZ(Value, Value),
}

impl Instruction {
    fn execute(&self, execution_index: &mut isize, register: &mut Register, sound_stack: &mut SoundStack) {
        println!("EXE:\n\t{:?}", self);

        match self {
            &SND(ref x) => sound_stack.push(x.resolve(register)),
            &SET(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(register);
                register[x] = y;
            }
            &ADD(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(register);
                register[x] += y;
            }
            &MUL(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(register);
                register[x] *= y;
            }
            &MOD(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(register);
                register[x] %= y;
            }
            &RCV(ref x) => {
                let x = x.resolve(register);
                if x != 0 {
                    println!("RCV:\n\t{:?}", sound_stack);
                    println!("\n\nAnswer: {}", sound_stack.last().unwrap());
                    process::exit(0);
                }
            }
            &JGZ(ref x, ref y) => {
                let x = x.resolve(register);
                let y = y.resolve(register) as isize;
                if x > 0 {
                    *execution_index += y;
                    return;
                }
            }
        }
        *execution_index += 1;
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        Ok(match iter.next() {
            Some("snd") => {
                let x = get_single_value(iter);
                SND(x)
            }
            Some("set") => {
                let (x, y) = get_double_value(iter);
                SET(x, y)
            }
            Some("add") => {
                let (x, y) = get_double_value(iter);
                ADD(x, y)
            }
            Some("mul") => {
                let (x, y) = get_double_value(iter);
                MUL(x, y)
            }
            Some("mod") => {
                let (x, y) = get_double_value(iter);
                MOD(x, y)
            }
            Some("rcv") => {
                let x = get_single_value(iter);
                RCV(x)
            }
            Some("jgz") => {
                let (x, y) = get_double_value(iter);
                JGZ(x, y)
            }
            _ => panic!("invalid instruction: {}", s),
        })
    }
}

fn get_single_value<'a, T: Iterator<Item=&'a str>>(mut iter: T) -> Value {
    iter.next().unwrap().parse().unwrap()
}

fn get_double_value<'a, T: Iterator<Item=&'a str>>(mut iter: T) -> (Value, Value) {
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().parse().unwrap();
    (x, y)
}

#[derive(Debug)]
enum Value {
    Register(usize),
    Raw(i64),
}

impl Value {
    fn get(&self) -> usize {
        match self {
            &Register(i) => i,
            &Raw(_) => panic!("tried to get a raw"),
        }
    }

    fn resolve(&self, register: &Register) -> i64 {
        match self {
            &Register(i) => register[i],
            &Raw(n) => n,
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