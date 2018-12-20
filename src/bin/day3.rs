#![feature(test)]

use aoc2018::*;
use std::collections::HashMap;

const INPUT: &str = include_str!("data/input_day3.txt");

fn solve_a(seq: &str) -> usize {
    let mut fabric = HashMap::new();
    let parser = |s: &str| {
        s.split(|c| c == ',' || c == 'x')
            .map(|x| x.trim_matches(|x: char| !x.is_numeric()))
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };
    for claim in seq.lines() {
        match claim
            .split_whitespace()
            .skip(2)
            .flat_map(parser)
            .collect::<Vec<_>>()
            .as_slice()
        {
            [a, b, c, d] => {
                let (a, b, c, d) = (*a, *b, *c, *d);
                for row in (b + 1)..(b + 1 + d) {
                    for col in (a + 1)..(a + 1 + c) {
                        *fabric.entry((row, col)).or_insert(0) += 1
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    fabric.values().filter(|&&v| v > 1).count()
}

fn solve_b(seq: &str) -> String {
    unimplemented!()
}

fn main() {
    println!("{}", solve_a(INPUT));
    //println!("{:?}", solve_b(INPUT));
}

test!(101469);
bench!(A);
