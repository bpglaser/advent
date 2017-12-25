use std::collections::HashMap;

// 1114687 too high

fn main() {
    let mut machine = Machine::new();
    for _ in 0..12261543 {
        machine.step();
    }
    println!("Checksum: {}", machine.checksum());
}

#[derive(Clone, Copy)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Machine {
    state: State,
    index: isize,
    tape: HashMap<isize, u8>,
}

impl Machine {
    fn new() -> Self {
        Self {
            state: State::A,
            index: 0,
            tape: HashMap::new(),
        }
    }

    fn step(&mut self) {
        match self.state {
            State::A => {
                if self.get() == 0 {
                    self.set(1);
                    self.index += 1;
                    self.state = State::B;
                } else {
                    self.set(0);
                    self.index -= 1;
                    self.state = State::C;
                }
            }
            State::B => {
                if self.get() == 0 {
                    self.set(1);
                    self.index -= 1;
                    self.state = State::A;
                } else {
                    self.set(1);
                    self.index += 1;
                    self.state = State::C;
                }
            }
            State::C => {
                if self.get() == 0 {
                    self.set(1);
                    self.index += 1;
                    self.state = State::A;
                } else {
                    self.set(0);
                    self.index -= 1;
                    self.state = State::D;
                }
            }
            State::D => {
                if self.get() == 0 {
                    self.set(1);
                    self.index -= 1;
                    self.state = State::E;
                } else {
                    self.set(1);
                    self.index -= 1;
                    self.state = State::C;
                }
            }
            State::E => {
                if self.get() == 0 {
                    self.set(1);
                    self.index += 1;
                    self.state = State::F;
                } else {
                    self.set(1);
                    self.index += 1;
                    self.state = State::A;
                }
            }
            State::F => {
                if self.get() == 0 {
                    self.set(1);
                    self.index += 1;
                    self.state = State::A;
                } else {
                    self.set(1);
                    self.index += 1;
                    self.state = State::E;
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        self.tape.values().filter(|v| **v == 1).count()
    }

    fn get(&mut self) -> u8 {
        if !self.tape.contains_key(&self.index) {
            self.tape.insert(self.index, 0);
        }
        *self.tape.get(&self.index).unwrap()
    }

    fn set(&mut self, value: u8) {
        self.tape.insert(self.index, value);
    }
}
