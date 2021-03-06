#![feature(test)]

use benchtest::benchtest;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("data/input_day2.txt");

fn solve_a(seq: &str) -> u64 {
    let (mut twos, mut threes) = (0, 0);
    for id in seq.lines() {
        let mut counts = HashMap::new();
        for letter in id.chars() {
            let count = counts.entry(letter).or_insert(0);
            *count += 1;
        }
        twos += counts.values().any(|v| *v == 2) as u64;
        threes += counts.values().any(|v| *v == 3) as u64;
    }
    twos * threes
}

fn solve_b(seq: &str) -> String {
    let mut common = None;
    for pair in seq.lines().combinations(2) {
        let char_pairs = pair[0].chars().zip(pair[1].chars());
        let s = char_pairs
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .collect::<String>();
        if s.len() == 25 {
            common = Some(s);
            break;
        }
    }
    common.unwrap()
}

fn main() {
    println!("{:?}", solve_a(INPUT));
    println!("{:?}", solve_b(INPUT));
}

benchtest! {
    part_a_test: solve_a(INPUT) => 5434,
    part_b_test: solve_b(INPUT) => "agimdjvlhedpsyoqfzuknpjwt"
}
