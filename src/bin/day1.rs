#![feature(test)]

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_a() {
        assert_eq!(solve_a(INPUT), 493);
    }
    #[test]
    fn part_b() {
        assert_eq!(solve_b(INPUT), 413);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn part_a_bench(b: &mut Bencher) {
        b.iter(|| solve_a(INPUT));
    }

    #[bench]
    fn part_b_bench(b: &mut Bencher) {
        b.iter(|| solve_b(INPUT));
    }
}
