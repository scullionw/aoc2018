#![feature(test)]

use aoc2018::{test, bench};

const INPUT: &str = include_str!("data/input_day5.txt");

type Unit = char;

fn main() {
    println!("{:?}", solve_a(INPUT));
}

fn solve_a(seq: &str) -> usize {
    let mut polymer: Vec<(Unit, bool)> = seq.trim().chars().zip(std::iter::repeat(false)).collect();

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

test!(9386);
bench!(A);

