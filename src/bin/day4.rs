#![feature(test)]

use aoc2018::*;
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("data/input_day4.txt");

lazy_static! {
    static ref RECORD_RE: Regex = Regex::new(r"^\[(?P<timestamp>.*)\] (?P<activity>.*)$").unwrap();
    static ref TIMESTAMP_RE: Regex =
        Regex::new(r"^(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+) (?P<hour>\d+):(?P<minute>\d+)$")
            .unwrap();
    static ref ACTIVITY_RE: Regex =
        Regex::new(r"^(?P<variant>[a-zA-Z]+) (#(?P<id>\d+) )?(.*)$").unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TimeStamp {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
}

impl TimeStamp {
    fn from_str(s: &str) -> TimeStamp {
        let caps = TIMESTAMP_RE.captures(s).unwrap();
        TimeStamp {
            year: caps["year"].parse().unwrap(),
            month: caps["month"].parse().unwrap(),
            day: caps["day"].parse().unwrap(),
            hour: caps["hour"].parse().unwrap(),
            minute: caps["minute"].parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record {
    pub time: TimeStamp,
    pub activity: Activity,
}

impl Record {
    fn from_str(s: &str) -> Record {
        let caps = RECORD_RE.captures(s).unwrap();
        Record {
            time: TimeStamp::from_str(&caps["timestamp"]),
            activity: Activity::from_str(&caps["activity"]),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Activity {
    WakeUp,
    FallAsleep,
    BeginShift(u16),
}

impl Activity {
    fn from_str(s: &str) -> Activity {
        let caps = ACTIVITY_RE.captures(s).unwrap();
        match caps.name("id") {
            Some(id) => Activity::BeginShift(id.as_str().parse().unwrap()),
            None => match &caps["variant"] {
                "falls" => Activity::FallAsleep,
                "wakes" => Activity::WakeUp,
                _ => unreachable!(),
            },
        }
    }
}

fn solve_a(seq: &str) -> usize {
    unimplemented!()
}

fn solve_b(seq: &str) -> u32 {
    unimplemented!()
}

fn main() {
    // println!("{:?}", solve_a(INPUT));
    // println!("{:?}", solve_b(INPUT));
    let mut records = INPUT
        .lines()
        .take(10)
        .map(Record::from_str)
        .collect::<Vec<_>>();

    records.sort_unstable();
    for record in records {
        println!("{:?}", record)
    }
}

// test!(101469);
// bench!(A);
