use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use crate::Opcode::*;

type Registers = [usize; 6];

fn main() {
    let path = args().nth(1).expect("input path");
    let (ip, instructions) = load_input(&path);
    let mut reg = Registers::default();
    reg[0] = 1;

    while let Some(instruction) = instructions.get(reg[ip]) {
        // print!("ip={} {:?} {} ", reg[ip], reg, instruction.to_string());
        instruction.execute(&mut reg);
        // println!("{:?}", reg);
        reg[ip] += 1;

        if reg[ip] == 1 {
            break;
        }
    }

    println!("{}", sum_of_factors(reg[1]));
}

fn sum_of_factors(n: usize) -> usize {
    (1..=n).filter(|x| n % x == 0).sum()
}

fn load_input(path: &str) -> (usize, Vec<Instruction>) {
    let mut buf = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let first_line = lines.next().unwrap();
    let ip: usize = first_line.split_at(4).1.parse().unwrap();

    let instructions = lines.map(|s| s.parse().unwrap()).collect();

    (ip, instructions)
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

struct Instruction {
    op: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
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

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let op = match words.next().unwrap() {
            "addr" => ADDR,
            "addi" => ADDI,
            "mulr" => MULR,
            "muli" => MULI,
            "banr" => BANR,
            "bani" => BANI,
            "borr" => BORR,
            "bori" => BORI,
            "setr" => SETR,
            "seti" => SETI,
            "gtir" => GTIR,
            "gtri" => GTRI,
            "gtrr" => GTRR,
            "eqir" => EQIR,
            "eqri" => EQRI,
            "eqrr" => EQRR,
            unknown => panic!("unknown op: {}", unknown),
        };

        Ok(Instruction {
            op,
            a: words.next().unwrap().parse().unwrap(),
            b: words.next().unwrap().parse().unwrap(),
            c: words.next().unwrap().parse().unwrap(),
        })
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        let op = match self.op {
            ADDR => "addr",
            ADDI => "addi",
            MULR => "mulr",
            MULI => "muli",
            BANR => "banr",
            BANI => "bani",
            BORR => "borr",
            BORI => "bori",
            SETR => "setr",
            SETI => "seti",
            GTIR => "gtir",
            GTRI => "gtri",
            GTRR => "gtrr",
            EQIR => "eqir",
            EQRI => "eqri",
            EQRR => "eqrr",
        };
        format!("{} {} {} {}", op, self.a, self.b, self.c)
    }
}
