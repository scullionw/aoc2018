#![feature(test)]

use benchtest::benchtest;
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("data/input_day3.txt");
const FABRIC_SIZE: usize = 1000;

lazy_static! {
    static ref CLAIM_RE: Regex =
        Regex::new(r"^#(?P<id>\d+) @ (?P<col>\d+),(?P<row>\d+): (?P<w>\d+)x(?P<h>\d+)$").unwrap();
}

struct Claim {
    pub id: u32,
    pub row: usize,
    pub col: usize,
    pub h: usize,
    pub w: usize,
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

    fn locations(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = (self.row + 1)..(self.row + 1 + self.h);
        let cols = (self.col + 1)..(self.col + 1 + self.w);
        rows.flat_map(move |row| cols.clone().map(move |col| (row, col)))
    }
}

fn solve_a(seq: &str) -> usize {
    let mut fabric = [0u8; FABRIC_SIZE * FABRIC_SIZE];
    for claim in seq.lines().map(Claim::from_str) {
        for (row, col) in claim.locations() {
            let loc = row * FABRIC_SIZE + col;
            fabric[loc] = fabric[loc].saturating_add(1);
        }
    }
    fabric.iter().filter(|v| **v > 1).count()
}

fn solve_b(seq: &str) -> u32 {
    let mut fabric = [0u8; FABRIC_SIZE * FABRIC_SIZE];
    let claims = seq.lines().map(Claim::from_str).collect::<Vec<_>>();
    for claim in &claims {
        for (row, col) in claim.locations() {
            let loc = row * FABRIC_SIZE + col;
            fabric[loc] = fabric[loc].saturating_add(1);
        }
    }

    claims
        .iter()
        .find(|claim| {
            claim
                .locations()
                .all(|(row, col)| fabric[row * FABRIC_SIZE + col] == 1)
        })
        .unwrap()
        .id
}

fn main() {
    println!("{:?}", solve_a(INPUT));
    println!("{:?}", solve_b(INPUT));
}

benchtest! {
    part_a_test: solve_a(INPUT) => 101469,
    part_b_test: solve_b(INPUT) => 1067
}
