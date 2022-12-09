use std::{cell::Cell, cmp, collections::HashMap, env::args, fs::read_to_string};

use FileSystem::*;
use Token::*;

const TOTAL_FS_SIZE: usize = 70_000_000;
const TARGET_UNUSED: usize = 30_000_000;

#[derive(Debug, PartialEq)]
enum Token<'a> {
    CD(&'a str),
    LS,
    DirEntry(&'a str),
    FileEntry(usize, &'a str),
}

impl<'a> Token<'a> {
    fn lex(s: &'a str) -> Self {
        if s.starts_with("$ cd") {
            CD(&s["$ cd ".len()..])
        } else if s == "$ ls" {
            LS
        } else if s.starts_with("dir") {
            DirEntry(&s["dir ".len()..])
        } else {
            let i = s.find(' ').unwrap();
            FileEntry(s[..i].parse().unwrap(), &s[i + 1..])
        }
    }
}

#[derive(Debug)]
enum FileSystem {
    Directory {
        children: HashMap<String, FileSystem>,
        children_size: usize,
    },
    File(usize),
}

impl FileSystem {
    fn root() -> Self {
        let mut children = HashMap::new();
        children.insert("/".to_owned(), Self::empty_dir());
        Directory {
            children,
            children_size: 0,
        }
    }

    fn empty_dir() -> Self {
        Directory {
            children: HashMap::new(),
            children_size: 0,
        }
    }

    fn parse<'a, I>(mut self, mut iter: I) -> (Self, I)
    where
        I: Iterator<Item = Token<'a>>,
    {
        let Some(token) = iter.next() else {
            return (self, iter)
        };

        match token {
            CD(name) => {
                if name == ".." {
                    return (self, iter);
                }
                let dir = self
                    .children_mut()
                    .remove(name)
                    .expect("tried to CD into non-existent dir");
                *self.size_mut() -= dir.size();
                let (child, iter) = dir.parse(iter);
                *self.size_mut() += child.size();
                self.children_mut().insert(name.to_owned(), child);
                self.parse(iter)
            }
            LS => self.parse(iter),
            DirEntry(name) => {
                self.children_mut()
                    .entry(name.to_owned())
                    .or_insert(Self::empty_dir());
                self.parse(iter)
            }
            FileEntry(size, name) => {
                self.children_mut().insert(name.to_owned(), File(size));
                *self.size_mut() += size;
                self.parse(iter)
            }
        }
    }

    fn children_mut(&mut self) -> &mut HashMap<String, Self> {
        match self {
            Directory {
                ref mut children, ..
            } => children,
            File(_) => panic!("cannot get children of a file"),
        }
    }

    fn size(&self) -> usize {
        match self {
            Directory { children_size, .. } => *children_size,
            File(size) => *size,
        }
    }

    fn size_mut(&mut self) -> &mut usize {
        match self {
            Directory { children_size, .. } => children_size,
            File(size) => size,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Directory { .. } => true,
            File(_) => false,
        }
    }

    fn traverse<F: FnMut(&Self) -> () + Copy>(&self, mut f: F) {
        match self {
            Directory { children, .. } => children.values().for_each(|child| child.traverse(f)),
            File(_) => (),
        }
        f(self);
    }
}

fn solve(s: &str) -> String {
    let (fs, mut iter) = FileSystem::root().parse(s.lines().map(Token::lex));
    assert_eq!(iter.next(), None);
    let currently_free = TOTAL_FS_SIZE - fs.size();
    let min_valid_dir = Cell::new(usize::MAX);
    fs.traverse(|entry: &FileSystem| {
        let size = entry.size();
        if entry.is_dir() && currently_free + size >= TARGET_UNUSED {
            min_valid_dir.set(cmp::min(min_valid_dir.get(), size));
        }
    });
    format!("{}", min_valid_dir.get())
}

fn input() -> String {
    let path = args().nth(1).unwrap();
    read_to_string(path).unwrap()
}

fn main() {
    let s = input();
    let res = solve(&s);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let given = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(solve(&given), "24933642");
    }
}
