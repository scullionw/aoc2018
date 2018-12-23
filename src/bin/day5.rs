#![feature(test)]

use aoc2018::{test, bench};

const INPUT: &str = include_str!("data/input_day5.txt");

type Unit = char;

fn main() {
    println!("{:?}", solve_a(INPUT));
}

fn solve_a(seq: &str) -> usize {
    let mut polymer: Vec<Unit> = seq.trim().chars().collect();
    let mut marked = Vec::with_capacity(polymer.len());

    loop {
        let mut idx = 0;
        // Mark all (idx, idx + 1) pairs that will react
        while idx < polymer.len() - 1 {
            if opposite_polarity(polymer[idx], polymer[idx + 1]) {
                marked.push(idx);
                marked.push(idx + 1);
                // Skip the marked unit
                idx += 2;
            } else {
                idx += 1;
            }
        }

        if marked.is_empty() {
            break polymer.len();
        } else {
            // Iterate in reverse order so that all indexes
            // remain valid after remove()
            for i in marked.iter().rev() {
                polymer.remove(*i);
            }
            marked.clear();
        }
    }
}

fn opposite_polarity(u1: Unit, u2: Unit) -> bool {
    u1 != u2 && u1.eq_ignore_ascii_case(&u2)
}

test!(9386);
bench!(A);

