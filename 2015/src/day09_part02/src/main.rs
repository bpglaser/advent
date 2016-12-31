use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let links: Vec<Link> = load_lines().iter().map(|line| Link::parse(&line)).collect();

    let mut cities: HashSet<&str> = HashSet::new();
    for &Link { ref city_a, ref city_b, .. } in links.iter() {
        cities.insert(&city_a);
        cities.insert(&city_b);
    }
    let cities: Vec<&str> = cities.into_iter().collect();
    let permutations = gen_permutations(&cities);

    let mut best_path = None;
    let mut best_dist = 0;

    for permutation in permutations.iter() {
        let mut total_distance = 0;
        for pair in permutation.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let link = links.iter().find(|link| link.between(a, b)).expect("Unable to find link");
            total_distance += link.distance;
        }
        if total_distance > best_dist {
            best_path = Some(permutation);
            best_dist = total_distance;
        }
    }

    println!("Best path: {:?}", best_path.expect("Failed to find best path"));
    println!("Distance: {}", best_dist);
}

struct Link {
    city_a: String,
    city_b: String,
    distance: usize,
}

impl Link {
    fn parse(line: &str) -> Link {
        let words: Vec<_> = line.split_whitespace().collect();
        Link { city_a: words[0].to_owned(), city_b: words[2].to_owned(), distance: words[4].parse().unwrap() }
    }

    fn between(&self, a: &str, b: &str) -> bool {
        (self.city_a == a && self.city_b == b) || (self.city_a == b && self.city_b == a)
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
