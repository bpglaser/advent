use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).unwrap();
    let root = load_input(&path);
    println!("{:?}", root);
}

fn load_input(path: &str) -> Group {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.parse().unwrap()
}

#[derive(Debug)]
struct Group {
    depth: u32,
    children: Vec<Child>,
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Token::*;

        let tokens = s.chars().map(Token::from).collect();
        let mut stream = TokenStream::new(tokens);

        let mut root = GroupBuilder::new();
        let mut depth = 0;
        let mut garbage: Option<String> = None;

        loop {
            match stream.next() {
                GroupStart => depth += 1,
                GroupEnd => depth -= 1,
                CommaSeperator => {
                    if let Some(garbage) = garbage.as_mut() {
                        garbage.push(',');
                    }
                }
                GarbageStart => {
                    if garbage.is_none() {
                        garbage = Some(String::new());
                    } else {
                        garbage.as_mut().map(|s| s.push('<'));
                    }
                }
                GarbageEnd => {
                    let garbage = garbage.take().expect("vaild garbage to terminate");
                }
                Cancel => stream.skip(1),
                Char(c) => {
                    let garbage = garbage.as_mut().expect("valid garbage");
                    garbage.push(c);
                }
            }

            // Done parsing
            if depth == 0 {
                break;
            }
        }

        Ok(root.build())
    }
}

#[derive(Debug)]
enum Child {
    Garbage(String),
    Group(Group),
}

struct GroupBuilder {
    depth: Option<u32>,
}

impl GroupBuilder {
    fn new() -> Self {
        Self { depth: None }
    }

    fn depth(&mut self, depth: u32) {
        self.depth = Some(depth);
    }

    fn build(self) -> Group {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
enum Token {
    GroupStart,
    GroupEnd,
    CommaSeperator,
    GarbageStart,
    GarbageEnd,
    Cancel,
    Char(char),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        use Token::*;

        match c {
            '{' => GroupStart,
            '}' => GroupEnd,
            ',' => CommaSeperator,
            '<' => GarbageStart,
            '>' => GarbageEnd,
            '!' => Cancel,
            _ => Char(c),
        }
    }
}

struct TokenStream {
    index: usize,
    tokens: Vec<Token>,
}

impl TokenStream {
    fn new(tokens: Vec<Token>) -> Self {
        Self { index: 0, tokens }
    }

    fn next(&mut self) -> Token {
        let i = self.index;
        self.index += 1;
        self.tokens[i]
    }

    fn skip(&mut self, n: usize) {
        self.index += n;
    }
}
