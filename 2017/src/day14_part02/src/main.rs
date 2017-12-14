extern crate bit_vec;

use std::env::args;
use std::collections::HashSet;

use bit_vec::BitVec;

use day10::knot_hash as hash;

fn main() {
    let input = args().nth(1).expect("valid input arg");
    
   let all_bytes: Vec<u8> = (0..128)
        .map(|n| format!("{}-{}", input, n))
        .flat_map(|s| hash(s.as_bytes()))
        .collect();

    let grid = Grid::from_bytes(&all_bytes);

    #[cfg(feature = "debug_print")]
    debug_print_grid(&grid);

    let mut count = 0;
    let mut seen = HashSet::new();
    for y in 0..128 {
        for x in 0..128 {
            let point = (x, y);
            if seen.contains(&point) {
                continue;
            }
            if !grid.get(x, y).unwrap() {
                continue;
            }
            count += 1;

            let mut stack = vec![point];
            while let Some(point) = stack.pop() {
                if seen.contains(&point) {
                    continue;
                }
                seen.insert(point);
                for neighbor in grid.get_adjacent(point.0, point.1) {
                    stack.push(neighbor);
                }
            }
        }
    }
    println!("{}", count);
}

#[cfg(feature = "debug_print")]
fn debug_print_grid(grid: &Grid) {
    for y in 0..128 {
        for x in 0..128 {
            if grid.get(x, y).unwrap() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

type Point = (usize, usize);

struct Grid {
    bit_vec: BitVec,
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let bit_vec = BitVec::from_bytes(bytes);
        assert!(bit_vec.len() == (128 * 128));
        Self { bit_vec }
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x >= 128 || y >= 128 {
            return None;
        }
        self.bit_vec.get(x + y * 128)
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<Point> {
        let mut adjacent = vec![];
        if x > 0 && self.get(x - 1, y).unwrap_or(false) { // left
            adjacent.push((x - 1, y));
        }
        if self.get(x + 1, y).unwrap_or(false) { // right
            adjacent.push((x + 1, y));
        }
        if y > 0 && self.get(x, y - 1).unwrap_or(false) { // up
            adjacent.push((x, y - 1));
        }
        if self.get(x, y + 1).unwrap_or(false) { // down
            adjacent.push((x, y + 1));
        }
        adjacent
    }
}

mod day10 {
    pub fn knot_hash(input: &[u8]) -> Vec<u8> {
        let mut input = Vec::from(input);
        input.extend_from_slice(&[17, 31, 73, 47, 23]);

        let mut sparse_hash: Vec<_> = (0..256u32).collect();
        let mut current_position = 0;
        let mut skip_size = 0;

        for _ in 0..64 {
            for length in &input {
                let length = *length as usize;
                let mut i = current_position;

                let mut temp: Vec<_> = sparse_hash
                    .iter()
                    .cloned()
                    .cycle()
                    .skip(current_position)
                    .take(length)
                    .collect();
                temp.reverse();

                for n in temp {
                    sparse_hash[i] = n;
                    i = (i + 1) % sparse_hash.len();
                }

                current_position = (current_position + length + skip_size) % sparse_hash.len();
                skip_size += 1;
            }
        }

        let mut dense_hash: Vec<u8> = vec![];
        for chunk in sparse_hash.chunks(16) {
            let mut n = chunk[0];
            for b in chunk.iter().skip(1) {
                n ^= b;
            }
            dense_hash.push(n as u8);
        }

        dense_hash
    }
}