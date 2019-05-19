#![feature(test)]

use benchtest::benchtest;
use std::iter;

const INPUT: &str = include_str!("data/input_day5.txt");

type Unit = char;
type Polymer = Vec<(Unit, bool)>;

fn main() {
    println!("{:?}", solve_a(INPUT));
    println!("{:?}", solve_b(INPUT));
}

fn solve_a(seq: &str) -> usize {
    let polymer = seq.trim().chars().zip(iter::repeat(false)).collect();
    collapse(polymer)
}

fn solve_b(seq: &str) -> usize {
    let polymer: Polymer = seq.trim().chars().zip(iter::repeat(false)).collect();
    let units = (0..26).map(|x| ((x + b'a') as Unit, (x + b'A') as Unit));

    units
        .map(|(u1, u2)| {
            let mut improved = polymer.clone();
            improved.retain(|&(u, _)| u != u1 && u != u2);
            collapse(improved)
        })
        .min()
        .unwrap()
}

fn collapse(mut polymer: Polymer) -> usize {
    loop {
        let mut idx = 0;
        let mut marked = false;
        // Mark all (idx, idx + 1) pairs that will react
        while idx < polymer.len() - 1 {
            if opposite_polarity(polymer[idx].0, polymer[idx + 1].0) {
                polymer[idx].1 = true;
                polymer[idx + 1].1 = true;
                marked = true;
                // Skip the marked unit
                idx += 2;
            } else {
                idx += 1;
            }
        }
        // Remove all units that were marked in this pass, or return length
        if marked {
            polymer.retain(|(_, marked)| !marked);
        } else {
            break polymer.len();
        }
    }
}

fn opposite_polarity(u1: Unit, u2: Unit) -> bool {
    u1 != u2 && u1.eq_ignore_ascii_case(&u2)
}

benchtest! {
    part_a_test: solve_a(INPUT) => 9386,
    part_b_test: solve_b(INPUT) => 4876
}
