use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::Read;

use crate::Opcode::*;

type Registers = [usize; 4];

fn main() {
    let path = args().nth(1).expect("input path");
    let (traces, raw_instructions) = load_input(&path);

    let mut op_choices: HashMap<usize, HashSet<Opcode>> = HashMap::new();
    for trace in traces {
        for op in Opcode::iter() {
            let mut reg = trace.before.clone();

            let instruction = Instruction::from_op(op, &trace.raw_instruction[1..]);
            instruction.execute(&mut reg);

            if reg == trace.after {
                let raw_op = trace.raw_instruction[0];
                if !op_choices.contains_key(&raw_op) {
                    op_choices.insert(raw_op, HashSet::new());
                }
                op_choices.get_mut(&raw_op).unwrap().insert(op);
            }
        }
    }

    let mut assigned: HashSet<usize> = HashSet::new();
    while op_choices.values().any(|choices| choices.len() > 1) {
        let mut singles = HashSet::new();

        for (op_num, ops) in &op_choices {
            if assigned.contains(op_num) {
                continue;
            } else if ops.len() == 1 {
                let op = *ops.iter().next().unwrap();
                singles.insert(op);
                assigned.insert(*op_num);
            }
        }

        for (op_num, ops) in &mut op_choices {
            if assigned.contains(&op_num) {
                continue;
            }
            for single in &singles {
                ops.remove(single);
            }
        }
    }

    let op_map: HashMap<usize, Opcode> = op_choices
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect();

    let mut reg = Registers::default();

    for raw in raw_instructions {
        let op = op_map[&raw[0]];
        let instruction = Instruction::from_op(op, &raw[1..]);
        instruction.execute(&mut reg);
    }

    println!("{}", reg[0]);
}

fn load_input(path: &str) -> (Vec<Trace>, Vec<[usize; 4]>) {
    let mut input = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut traces = vec![];
    let mut i = 0;
    while !lines[i].is_empty() {
        let before = parse_registers(lines[i]);
        let raw_instruction = parse_usize4(lines[i + 1]);
        let after = parse_registers(lines[i + 2]);

        traces.push(Trace {
            before,
            raw_instruction,
            after,
        });

        i += 4
    }

    (
        traces,
        lines.into_iter().skip(i + 2).map(parse_usize4).collect(),
    )
}

fn parse_registers(s: &str) -> Registers {
    let s: String = s
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .take_while(|c| c != &']')
        .filter(|c| c != &',')
        .collect();

    parse_usize4(&s)
}

fn parse_usize4(s: &str) -> [usize; 4] {
    let mut nums = s.split_whitespace().map(|n| n.parse().unwrap());

    [
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    ]
}

#[derive(Debug)]
struct Trace {
    before: Registers,
    raw_instruction: [usize; 4],
    after: Registers,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Opcode {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

impl Opcode {
    fn iter() -> impl Iterator<Item = Opcode> {
        [
            ADDR, ADDI, MULR, MULI, BANR, BANI, BORR, BORI, SETR, SETI, GTIR, GTRI, GTRR, EQIR,
            EQRI, EQRR,
        ]
        .into_iter()
        .cloned()
    }
}

struct Instruction {
    op: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn from_op(op: Opcode, raw: &[usize]) -> Self {
        Self {
            op,
            a: raw[0],
            b: raw[1],
            c: raw[2],
        }
    }

    fn execute(&self, reg: &mut Registers) {
        match self.op {
            ADDR => {
                reg[self.c] = reg[self.a] + reg[self.b];
            }
            ADDI => {
                reg[self.c] = reg[self.a] + self.b;
            }
            MULR => {
                reg[self.c] = reg[self.a] * reg[self.b];
            }
            MULI => {
                reg[self.c] = reg[self.a] * self.b;
            }
            BANR => {
                reg[self.c] = reg[self.a] & reg[self.b];
            }
            BANI => {
                reg[self.c] = reg[self.a] & self.b;
            }
            BORR => {
                reg[self.c] = reg[self.a] | reg[self.b];
            }
            BORI => {
                reg[self.c] = reg[self.a] | self.b;
            }
            SETR => {
                reg[self.c] = reg[self.a];
            }
            SETI => {
                reg[self.c] = self.a;
            }
            GTIR => {
                if self.a > reg[self.b] {
                    reg[self.c] = 1;
                } else {
                    reg[self.c] = 0;
                }
            }
            GTRI => {
                if reg[self.a] > self.b {
                    reg[self.c] = 1;
                } else {
                    reg[self.c] = 0;
                }
            }
            GTRR => {
                if reg[self.a] > reg[self.b] {
                    reg[self.c] = 1;
                } else {
                    reg[self.c] = 0;
                }
            }
            EQIR => {
                if self.a == reg[self.b] {
                    reg[self.c] = 1;
                } else {
                    reg[self.c] = 0;
                }
            }
            EQRI => {
                if reg[self.a] == self.b {
                    reg[self.c] = 1;
                } else {
                    reg[self.c] = 0;
                }
            }
            EQRR => {
                if reg[self.a] == reg[self.b] {
                    reg[self.c] = 1;
                } else {
                    reg[self.c] = 0;
                }
            }
        }
    }
}
