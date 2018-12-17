use std::collections::HashSet;

const INPUT: &str = include_str!("data/input_day1.txt");

fn solve_a(seq: &str) -> i64 {
    seq.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .sum()
}

fn solve_b(seq: &str) -> i64 {
    let mut set = HashSet::new();
    seq.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .cycle()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .find(|x| !set.insert(*x))
        .unwrap()
}

fn main() {
    println!("{}", solve_a(INPUT));
    println!("{}", solve_b(INPUT));
}
