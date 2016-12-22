use std::env::args;
use std::fs::File;
use std::io::Read;

use Instruction::*;

fn main() {
    let (password, path) = get_args();
    let mut password_chars = password.chars().collect();
    let instructions = load_instructions(&path);
    for instruction in instructions.iter() {
        instruction.execute(&mut password_chars);
    }
    let answer: String = password_chars.into_iter().collect();
    println!("scrabled password: {}", answer);
}

#[derive(Debug)]
enum Instruction {
    SwapPosition { x: usize, y: usize },
    SwapLetter { x: char, y: char },
    RotateSteps { count: isize },
    RotateLetter { letter: char },
    ReverseRange { x: usize, y: usize },
    MoveLetter { x: usize, y: usize },
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "swap" => {
                if words[1] == "position" {
                    let x = words[2].parse().unwrap();
                    let y = words[5].parse().unwrap();
                    return SwapPosition { x: x, y: y };
                } else {
                    let x = words[2].chars().next().unwrap();
                    let y = words[5].chars().next().unwrap();
                    return SwapLetter { x: x, y: y };
                }
            }
            "rotate" => {
                if words[1] == "based" {
                    let letter = words[6].chars().next().unwrap();
                    return RotateLetter { letter: letter };
                } else {
                    let mut n = words[2].parse().unwrap();
                    if words[1] == "left" {
                        n *= -1;
                    }
                    return RotateSteps { count: n };
                }
            }
            "reverse" => {
                let x = words[2].parse().unwrap();
                let y = words[4].parse().unwrap();
                return ReverseRange { x: x, y: y };
            }
            "move" => {
                let x = words[2].parse().unwrap();
                let y = words[5].parse().unwrap();
                return MoveLetter { x: x, y: y };
            }
            _ => panic!(),
        }
    }

    fn execute(&self, password: &mut Vec<char>) {
        print!("Preforming {:?} on {:?} ",
               self,
               password.iter().cloned().collect::<String>());
        match self {
            &SwapPosition { x, y } => password.swap(x, y),
            &SwapLetter { x, y } => {
                let pos_x = index_of(password, x);
                let pos_y = index_of(password, y);
                password.swap(pos_x, pos_y);
            }
            &RotateSteps { count } => {
                if count > 0 {
                    for _ in 0..count {
                        let c = password.pop().unwrap();
                        password.insert(0, c);
                    }
                } else {
                    for _ in 0..count.abs() {
                        let c = password.remove(0);
                        password.push(c);
                    }
                }
            }
            &RotateLetter { letter } => {
                let mut i = index_of(password, letter);
                if i >= 4 {
                    i += 1;
                }
                i += 1;
                for _ in 0..i {
                    let c = password.pop().unwrap();
                    password.insert(0, c);
                }
            }
            &ReverseRange { mut x, mut y } => {
                if x > y {
                    let a = x;
                    x = y;
                    y = a;
                }
                let buf: Vec<char> = password.drain(x..(y + 1)).collect();
                for c in buf {
                    password.insert(x, c);
                }
            }
            &MoveLetter { x, y } => {
                let c = password.remove(x);
                password.insert(y, c);
            }
        }
        println!("=>\t{}", password.iter().cloned().collect::<String>());
    }
}

fn index_of(word: &Vec<char>, c: char) -> usize {
    let mut index = 0;
    for letter in word {
        if letter == &c {
            return index;
        }
        index += 1;
    }
    panic!("Unable to find: {}", c)
}

fn load_instructions(path: &str) -> Vec<Instruction> {
    let mut file = File::open(path).expect(&format!("Invalid path: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect(&format!("Unable to read: {}", path));
    buf.lines().map(Instruction::from_line).collect()
}

fn get_args() -> (String, String) {
    let mut args = args().skip(1);
    (args.next().unwrap(), args.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    use super::Instruction::*;

    #[test]
    fn swap_position_test() {
        helper("abcd", "bacd", SwapPosition { x: 0, y: 1 });
        helper("abcd", "cbad", SwapPosition { x: 0, y: 2 });
        helper("abcd", "dbca", SwapPosition { x: 0, y: 3 });

        helper("abcd", "dbca", SwapPosition { x: 3, y: 0 });
        helper("abcd", "cbad", SwapPosition { x: 2, y: 0 });
        helper("abcd", "bacd", SwapPosition { x: 1, y: 0 });
    }

    #[test]
    fn swap_letter_test() {
        helper("abcd", "dbca", SwapLetter { x: 'a', y: 'd' });
        helper("abcd", "dbca", SwapLetter { x: 'd', y: 'a' });

        helper("abcd", "acbd", SwapLetter { x: 'b', y: 'c' });
        helper("abcd", "acbd", SwapLetter { x: 'c', y: 'b' });
    }

    #[test]
    fn rotate_steps_test() {
        helper("abcd", "abcd", RotateSteps { count: 0 });

        helper("abcd", "dabc", RotateSteps { count: 1 });
        helper("abcd", "bcda", RotateSteps { count: -1 });

        helper("abcd", "abcd", RotateSteps { count: 4 });
        helper("abcd", "abcd", RotateSteps { count: -4 });

        helper("abcd", "dabc", RotateSteps { count: 5 });
        helper("abcd", "bcda", RotateSteps { count: -5 });
    }

    #[test]
    fn rotate_letter_test() {
        helper("abcdef", "fabcde", RotateLetter { letter: 'a' });
        helper("abcdef", "efabcd", RotateLetter { letter: 'b' });
        helper("abcdef", "defabc", RotateLetter { letter: 'c' });
        helper("abcdef", "cdefab", RotateLetter { letter: 'd' });

        // index 4 rule applies below
        helper("abcdef", "abcdef", RotateLetter { letter: 'e' });
        helper("abcdef", "fabcde", RotateLetter { letter: 'f' });
    }

    #[test]
    fn reverse_range_test() {
        helper("abcd", "dcba", ReverseRange { x: 0, y: 3 });
        helper("abcd", "dcba", ReverseRange { x: 3, y: 0 });

        helper("abcd", "acbd", ReverseRange { x: 1, y: 2 });
        helper("abcd", "acbd", ReverseRange { x: 2, y: 1 });

        helper("abcdef", "aedcbf", ReverseRange { x: 1, y: 4 });

        helper("abcdef", "adcbef", ReverseRange { x: 3, y: 1 });
    }

    #[test]
    fn move_letter_test() {
        helper("abcd", "bcda", MoveLetter { x: 0, y: 3 });
        helper("abcd", "dabc", MoveLetter { x: 3, y: 0 });

        helper("abcd", "acdb", MoveLetter { x: 1, y: 3 });
        helper("abcd", "adbc", MoveLetter { x: 3, y: 1 });
    }

    fn helper(source: &str, target: &str, instruction: Instruction) {
        let mut word: Vec<char> = source.chars().collect();
        instruction.execute(&mut word);
        assert_eq!(target, &word.iter().cloned().collect::<String>());
    }
}
