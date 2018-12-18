use std::collections::HashSet;

const INPUT: &str = include_str!("data/input_day1.txt");

fn solve_a(seq: &str) -> i64 {
    seq.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .sum()
}

fn solve_b(seq: &str) -> i64 {
    let mut reached = HashSet::new();
    seq.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .cycle()
        .scan(0, |csum, x| {
            *csum += x;
            Some(*csum)
        })
        .find(|x| !reached.insert(*x))
        .unwrap()
}

fn main() {
    println!("{}", solve_a(INPUT));
    println!("{}", solve_b(INPUT));
}
