use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).unwrap();
    let machine = load_machine(&path);
    let result = machine.execute();
    let max = result.iter().max_by_key(|&(_, v)| v).unwrap();
    println!("MAX:\t{:?}", max);
}

fn load_machine(path: &str) -> Machine {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let instructions = buf.lines().map(|s| s.parse().unwrap()).collect();
    Machine::from_instructions(instructions)
}

type Register = HashMap<String, i32>;

#[derive(Debug)]
struct Machine {
    register: Register,
    instructions: Vec<Instruction>,
}

impl Machine {
    fn new(register: HashMap<String, i32>, instructions: Vec<Instruction>) -> Self {
        Machine {
            register,
            instructions,
        }
    }

    fn from_instructions(instructions: Vec<Instruction>) -> Self {
        let register = HashMap::new();
        Machine::new(register, instructions)
    }

    fn execute(mut self) -> Register {
        for instruction in self.instructions {
            instruction.execute(&mut self.register);
        }
        self.register
    }
}

#[derive(Debug)]
struct Instruction {
    register: String,
    operator: Operator,
    offset: i32,
    condition: Condition,
}

impl Instruction {
    fn execute(&self, register: &mut Register) {
        use Operator::*;

        if self.condition.evaluate(register) {
            let reg_val = {
                if !register.contains_key(&self.register) {
                    register.insert(self.register.clone(), 0);
                }
                register.get_mut(&self.register).unwrap()
            };
            match &self.operator {
                &Inc => (*reg_val) += self.offset,
                &Dec => (*reg_val) -= self.offset,
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        let register = words[0].to_owned();
        let operator = words[1].parse().unwrap();
        let offset = words[2].parse().unwrap();

        let cond_reg = words[4].to_owned();
        let cond_comp = words[5].parse().unwrap();
        let cond_off = words[6].parse().unwrap();

        let condition = Condition::new(cond_reg, cond_comp, cond_off);

        Ok(Instruction {
            register,
            operator,
            offset,
            condition,
        })
    }
}

#[derive(Debug)]
enum Operator {
    Inc,
    Dec,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operator::*;
        Ok(match &s {
            &"inc" => Inc,
            &"dec" => Dec,
            _ => panic!("invalid operator: {}", s),
        })
    }
}

#[derive(Debug)]
struct Condition {
    register: String,
    comparator: Comparator,
    constant: i32,
}

impl Condition {
    fn new(register: String, comparator: Comparator, constant: i32) -> Self {
        Condition {
            register,
            comparator,
            constant,
        }
    }

    fn evaluate(&self, register: &mut Register) -> bool {
        if !register.contains_key(&self.register) {
            register.insert(self.register.clone(), 0);
        }
        let val = *register.get(&self.register).unwrap();
        self.comparator.evaluate(val, self.constant)
    }
}

#[derive(Debug)]
enum Comparator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreatherThanOrEqualTo,
    LessThanOrEqualTo,
}

impl Comparator {
    fn evaluate(&self, left: i32, right: i32) -> bool {
        use Comparator::*;

        match self {
            &Equal => left == right,
            &NotEqual => left != right,
            &GreaterThan => left > right,
            &LessThan => left < right,
            &GreatherThanOrEqualTo => left >= right,
            &LessThanOrEqualTo => left <= right,
        }
    }
}

impl FromStr for Comparator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Comparator::*;
        Ok(match &s {
            &"==" => Equal,
            &"!=" => NotEqual,
            &">" => GreaterThan,
            &"<" => LessThan,
            &">=" => GreatherThanOrEqualTo,
            &"<=" => LessThanOrEqualTo,
            _ => panic!("invalid comparator: {}", s),
        })
    }
}
