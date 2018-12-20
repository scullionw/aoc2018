#![feature(test)]

use aoc2018::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("data/input_day3.txt");

lazy_static! {
    static ref CLAIM_RE: Regex =
        Regex::new(r"^#(?P<id>\d+) @ (?P<col>\d+),(?P<row>\d+): (?P<w>\d+)x(?P<h>\d+)$").unwrap();
}

struct Claim {
    pub id: u32,
    pub row: u32,
    pub col: u32,
    pub h: u32,
    pub w: u32,
}

impl Claim {
    fn from_str(s: &str) -> Claim {
        let caps = CLAIM_RE.captures(s).unwrap();
        Claim {
            id: caps["id"].parse().unwrap(),
            row: caps["row"].parse().unwrap(),
            col: caps["col"].parse().unwrap(),
            h: caps["h"].parse().unwrap(),
            w: caps["w"].parse().unwrap(),
        }
    }

    fn locations(&self) -> impl Iterator<Item = (u32, u32)> {
        let rows = (self.row + 1)..(self.row + 1 + self.h);
        let cols = (self.col + 1)..(self.col + 1 + self.w);
        rows.flat_map(move |row| cols.clone().map(move |col| (row, col)))
    }
}

fn solve_a(seq: &str) -> usize {
    let mut fabric = HashMap::new();
    let claims = seq.lines().map(Claim::from_str).collect::<Vec<_>>();
    for claim in claims {
        for (row, col) in claim.locations() {
            *fabric.entry((row, col)).or_insert(0) += 1
        }
    }
    fabric.values().filter(|v| **v > 1).count()
}

fn solve_b(seq: &str) -> u32 {
    let mut fabric = HashMap::new();
    let claims = seq.lines().map(Claim::from_str).collect::<Vec<_>>();
    for claim in &claims {
        for (row, col) in claim.locations() {
            *fabric.entry((row, col)).or_insert(0) += 1;
        }
    }

    claims
        .iter()
        .find(|claim| claim.locations().all(|loc| fabric[&loc] == 1))
        .unwrap()
        .id
}

fn main() {
    println!("{:?}", solve_a(INPUT));
    println!("{:?}", solve_b(INPUT));
}

test!(101469);
bench!(A, B);
