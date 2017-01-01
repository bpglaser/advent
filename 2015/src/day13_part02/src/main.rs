use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let links: Vec<Link> = load_lines().iter().map(|s| Link::parse(s)).collect();

    let mut persons: HashSet<&str> = HashSet::new();
    for link in links.iter() {
        persons.insert(&link.person_a);
        persons.insert(&link.person_b);
    }
    let mut persons: Vec<&str> = persons.into_iter().collect();
    persons.push("self");

    let mut best_affinity = isize::min_value();

    for permutation in gen_permutations(&persons) {
        let mut total_affinity = 0;

        for pair in permutation.windows(2) {
            if *pair[0] == "self" || *pair[1] == "self" {
                continue;
            }

            let link = find_link(&links, pair[0], pair[1]);
            total_affinity += link.affinity;

            let link = find_link(&links, pair[1], pair[0]);
            total_affinity += link.affinity;
        }

        let a = permutation.first().unwrap();
        let b = permutation.last().unwrap();

        if **a != "self" && **b != "self" {
            let link = find_link(&links, a, b);
            total_affinity += link.affinity;

            let link = find_link(&links, b, a);
            total_affinity += link.affinity;
        }

        if best_affinity < total_affinity {
            best_affinity = total_affinity;
        }
    }

    println!("Optimal change in happiness: {}", best_affinity);
}

fn find_link<'a>(links: &'a [Link], a: &str, b: &str) -> &'a Link {
    links.iter().find(|l| l.between(a, b)).expect("Unable to find pair")
}

#[derive(Debug)]
struct Link {
    person_a: String,
    person_b: String,
    affinity: isize,
}

impl Link {
    fn parse(line: &str) -> Link {
        let words: Vec<_> = line.split_whitespace().collect();
        let person_a = words[0].to_owned();
        let person_b = (*words.last().unwrap()).trim_right_matches('.').to_owned();
        let mut affinity = words[3].parse().unwrap();
        if words[2] == "lose" {
            affinity *= -1;
        }
        Link { person_a: person_a, person_b: person_b, affinity: affinity }
    }

    fn between(&self, a: &str, b: &str) -> bool {
        (self.person_a == a && self.person_b == b)
    }
}

// Permutations generated via Heap's Algorithm
fn gen_permutations<'a, T>(items: &'a [T]) -> Vec<Vec<&'a T>> {
    let mut item_refs: Vec<&T> = items.iter().collect();
    let mut c = vec![0; items.len()];

    let mut all_permutations = vec![item_refs.clone()];

    let mut i = 0;
    while i < items.len() {
        if c[i] < i {
            if i % 2 == 0 {
                item_refs.swap(0, i);
            } else {
                item_refs.swap(c[i], i);
            }
            all_permutations.push(item_refs.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    all_permutations
}

fn load_lines() -> Vec<String> {
    let mut file = File::open(args().nth(1).expect("Invalid args")).expect("Error opening file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");
    buf.lines().map(|s| s.trim().to_owned()).collect()
}
