use std::cell::RefCell;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let filename = args().skip(1).next().expect("Please pass a filename");

    let mut bots = vec![];
    let mut disbursements = vec![];

    for line in load_lines(&filename) {
        match line.parse().expect("Error parsing line") {
            Instruction::Bot(bot) => bots.push(Rc::new(RefCell::new(bot))),
            Instruction::Disbursement(disbursement) => disbursements.push(disbursement),
        }
    }

    while let Some(disbursement) = disbursements.pop() {
        let bot = find_bot(disbursement.identifier, &bots);
        let mut bot = bot.borrow_mut();
        match bot.pass(disbursement.value) {
            Some((low, high)) => {
                match bot.low_target {
                    Target::Bot(identifier) => disbursements.push(Disbursement { identifier: identifier, value: low }),
                    Target::Output(identifier) => println!("OUTPUT [{}] => {}", identifier, low),
                }
                match bot.high_target {
                    Target::Bot(identifier) => disbursements.push(Disbursement { identifier: identifier, value: high }),
                    Target::Output(identifier) => println!("OUTPUT [{}] => {}", identifier, high),
                }
            },
            None => {},
        };
    }
}

fn find_bot(identifier: isize, bots: &Vec<Rc<RefCell<Bot>>>) -> Rc<RefCell<Bot>> {
    bots.iter().find(|b| b.borrow().identifier == identifier).expect("Unable to find bot").clone()
}

#[derive(Clone)]
enum Target {
    Bot(isize),
    Output(isize),
}

#[derive(Clone)]
struct Bot {
    identifier: isize,
    low_target: Target,
    high_target: Target,
    buffer_value: Option<isize>,
}

impl Bot {
    fn new(identifier: isize, low_target: Target, high_target: Target) -> Bot {
        Bot {
            identifier: identifier,
            low_target: low_target,
            high_target: high_target,
            buffer_value: None,
        }
    }

    fn pass(&mut self, value: isize) -> Option<(isize, isize)> {
        match self.buffer_value {
            Some(n) => {
                if (n == 61 && value == 17) || (n == 17 && value == 61) {
                    println!("Comparision found: {}", self.identifier);
                }

                self.buffer_value = None;
                if value < n {
                    Some((value, n))
                } else {
                    Some((n, value))
                }
            },
            None => {
                self.buffer_value = Some(value);
                None
            },
        }
    }
}

struct Disbursement {
    identifier: isize,
    value: isize,
}

enum Instruction {
    Bot(Bot),
    Disbursement(Disbursement),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        match words[0] {
            "bot" => {
                let identifier = words[1].parse().unwrap();
                let low_num = words[6].parse().unwrap();
                let low_target = match words[5] {
                    "bot" => Target::Bot(low_num),
                    "output" => Target::Output(low_num),
                    _ => return Err(()),
                };
                let high_num = words[11].parse().unwrap();
                let high_target = match words[10] {
                    "bot" => Target::Bot(high_num),
                    "output" => Target::Output(high_num),
                    _ => return Err(()),
                };

                Ok(Instruction::Bot(Bot::new(identifier, low_target, high_target)))
            }
            "value" => {
                let value = words[1].parse().unwrap();
                let identifier = words[5].parse().unwrap();

                Ok(Instruction::Disbursement(Disbursement {
                    identifier: identifier,
                    value: value,
                }))
            }
            _ => Err(()),
        }
    }
}

fn load_lines(path: &str) -> Vec<String> {
    let mut f = File::open(path).expect(&format!("Error opening: {}", path));
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect(&format!("Error reading from: {}", path));
    buf.lines().map(|s| s.to_owned()).collect()
}
