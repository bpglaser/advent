#![feature(test)]
extern crate day21_part02;
extern crate test;

use day21_part02::do_puzzle;

use test::Bencher;

static INPUT: &str = include_str!("../input.txt");

#[bench]
fn single(b: &mut Bencher) {
    b.iter(|| do_puzzle(INPUT, 1));
}

#[bench]
fn many(b: &mut Bencher) {
    b.iter(|| do_puzzle(INPUT, 10));
}
