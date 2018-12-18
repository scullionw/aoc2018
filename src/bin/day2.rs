use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("data/input_day2.txt");

fn solve_a(seq: &str) -> u64 {
    let ids = seq.lines();
    let mut twos = 0;
    let mut threes = 0;
    for id in ids {
        twos += score(id, 2);
        threes += score(id, 3);
    }
    twos * threes
}

fn solve_b(seq: &str) -> String {
    let mut common = None;
    for pair in seq.lines().combinations(2) {
        assert!(pair.len() == 2);
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
    println!("{}", solve_a(INPUT));
    println!("{:?}", solve_b(INPUT));
}

fn score(id: &str, exactly: u64) -> u64 {
    let mut counts = HashMap::new();
    for letter in id.chars() {
        let count = counts.entry(letter).or_insert(0);
        *count += 1;
    }
    counts.retain(|_, v| *v == exactly);
    match counts.len() {
        0 => 0,
        _ => 1,
    }
}
