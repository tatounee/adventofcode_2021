#![allow(dead_code)]

mod input;
mod solutions;

use input::load_input;
use solutions::*;

fn main() {
    let input = load_input("input.txt").unwrap();
    let solution = day1::part1(&input);
    println!("Day {} - Puzzle {} : {}", 1, 1, solution);
}
