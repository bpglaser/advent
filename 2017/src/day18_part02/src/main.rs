use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::*;
use std::thread;

use Instruction::*;
use Value::*;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

type Register = Vec<i64>;
type ArcCount = Arc<AtomicIsize>;

struct DeadlockError;

fn main() {
    let input_path = args().nth(1).expect("valid input path");
    let instructions = load_instructions(&input_path);

    println!("Loaded instructions:");
    for instruction in &instructions {
        println!("\t{:?}", instruction);
    }

    let waiting_count = Arc::new(AtomicIsize::new(0));
    let (send0, recv0) = channel();
    let (send1, recv1) = channel();
    let send_count = Arc::new(AtomicIsize::new(0));

    let mut machine0 = Machine::new(0, instructions.clone(), send1, recv0, waiting_count.clone(), None);
    let mut machine1 = Machine::new(1, instructions, send0, recv1, waiting_count, Some(send_count.clone()));

    let handle0 = thread::spawn(move || machine0.execute());
    let handle1 = thread::spawn(move || machine1.execute());

    handle0.join().unwrap();
    handle1.join().unwrap();
    println!("Machine 1 sent: {} times", send_count.load(Ordering::Relaxed));
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
    sender: Sender<Signal>,
    receiver: Receiver<Signal>,
    waiting_count: ArcCount,
    send_count: Option<ArcCount>,
}

impl Machine {
    fn new(
        id: i64, 
        instructions: Vec<Instruction>, 
        sender: Sender<Signal>, 
        receiver: Receiver<Signal>, 
        waiting_count: ArcCount, 
        send_count: Option<ArcCount>) -> Self
    {
        let mut register = vec![0; LETTERS.len()];
        register[15] = id; // sets register 'p' to the id
        Self { id, execution_index: 0, instructions, register, sender, receiver, waiting_count, send_count }
    }

    fn execute(&mut self) {
        loop {
            if self.execution_index < 0 || self.execution_index as usize >= self.instructions.len() {
                panic!("Jumped out of bounds!");
            }

            let instruction = self.instructions[self.execution_index as usize];
            match instruction.execute(self){
                Ok(_) => {}
                Err(DeadlockError) => {
                    println!("[{}] Done executing", self.id);
                    return;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
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
    fn execute(&self, machine: &mut Machine) -> Result<(), DeadlockError> {
        println!("[{}] EXE:\n\t{:?}", machine.id, self);

        match self {
            &SND(ref x) => {
                if let Some(ref mut send_count) = machine.send_count {
                    send_count.fetch_add(1, Ordering::Relaxed);
                }
                machine.sender.send(Signal::Value(x.resolve(&mut machine.register))).expect("able to send");
            }
            &SET(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&mut machine.register);
                machine.register[x] = y;
            }
            &ADD(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&mut machine.register);
                machine.register[x] += y;
            }
            &MUL(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&mut machine.register);
                machine.register[x] *= y;
            }
            &MOD(ref x, ref y) => {
                let x = x.get() as usize;
                let y = y.resolve(&mut machine.register);
                machine.register[x] %= y;
            }
            &RCV(ref x) => {
                let x = x.get();
                match machine.receiver.try_recv() {
                    Ok(Signal::Value(y)) => machine.register[x] = y,
                    Ok(Signal::Deadlocked) => return Err(DeadlockError),
                    Err(_) => {
                        if machine.waiting_count.load(Ordering::Relaxed) == 1 {
                            println!("[{}] Some else is waiting. Exiting!", machine.id);
                            machine.sender.send(Signal::Deadlocked).unwrap();
                            return Err(DeadlockError);
                        }
                        println!("[{}] Waiting for value", machine.id);
                        machine.waiting_count.fetch_add(1, Ordering::Relaxed);
                        match machine.receiver.recv() {
                            Ok(Signal::Value(y)) => {
                                machine.waiting_count.fetch_sub(1, Ordering::Relaxed);
                                machine.register[x] = y;
                            }
                            _ => return Err(DeadlockError),
                        }
                    }
                }
            }
            &JGZ(ref x, ref y) => {
                let x = x.resolve(&mut machine.register);
                let y = y.resolve(&mut machine.register) as isize;
                if x > 0 {
                    machine.execution_index += y;
                    return Ok(());
                }
            }
        }
        machine.execution_index += 1;
        Ok(())
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

#[derive(Copy, Clone, Debug)]
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

enum Signal {
    Deadlocked,
    Value(i64),
}