#![feature(test)]

use benchtest::benchtest;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

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

#[derive(Debug)]
struct Day {
    id: u16,
    shift: Vec<u8>,
}

impl Day {
    fn from_records(records: &[&Record]) -> Day {
        let mut shift = vec![0u8; 60];
        let mut prev_time = 0;
        let mut guard_id = None;
        for record in records {
            match record.activity {
                Activity::BeginShift(id) => guard_id = Some(id),
                Activity::FallAsleep => {
                    let now = record.time.minute as usize;
                    for minute in &mut shift[prev_time..now] {
                        *minute = 0; // 0 is awake
                        prev_time = now;
                    }
                }
                Activity::WakeUp => {
                    let now = record.time.minute as usize;
                    for minute in &mut shift[prev_time..now] {
                        *minute = 1; // 1 is asleep
                        prev_time = now;
                    }
                }
            }
        }
        // Set awake until end of shift
        for minute in &mut shift[prev_time..] {
            *minute = 0;
        }
        Day {
            id: guard_id.unwrap(),
            shift,
        }
    }

    fn split_days(records: &mut [Record]) -> Vec<Day> {
        records.sort_unstable();
        let mut days = Vec::new();
        let mut day_records = Vec::new();
        for record in records.iter() {
            match record.activity {
                Activity::BeginShift(_) if !day_records.is_empty() => {
                    let day = Day::from_records(&day_records.drain(..).collect::<Vec<_>>());
                    days.push(day);
                    day_records.push(record)
                }
                _ => day_records.push(record),
            }
        }
        let day = Day::from_records(&day_records.drain(..).collect::<Vec<_>>());
        days.push(day);
        days
    }
}

fn solve_a(seq: &str) -> usize {
    let mut records = seq.lines().map(Record::from_str).collect::<Vec<_>>();
    let days = Day::split_days(&mut records);
    let mut counter = HashMap::new();
    for day in &days {
        *counter.entry(day.id).or_insert(0) += bytecount::count(&day.shift, 1); // use sum
    }
    let sleepiest_guard = counter
        .iter()
        .max_by_key(|&(_, time_asleep)| time_asleep)
        .unwrap()
        .0;

    let sleepy_guard_shifts = days
        .iter()
        .filter(|day| day.id == *sleepiest_guard)
        .map(|d| d.shift.clone());

    let mut shift_sum = vec![0; 60];
    for shift in sleepy_guard_shifts {
        for i in 0..60 {
            shift_sum[i] += shift[i];
        }
    }

    let sleepiest_minute = shift_sum
        .iter()
        .enumerate()
        .max_by_key(|&(_, el)| el)
        .unwrap()
        .0;

    (*sleepiest_guard as usize) * sleepiest_minute
}

fn solve_b(seq: &str) -> usize {
    let mut records = seq.lines().map(Record::from_str).collect::<Vec<_>>();
    let days = Day::split_days(&mut records);
    let mut counter = HashMap::new();
    for day in &days {
        let shift_sum = counter.entry(day.id).or_insert_with(|| vec![0u8; 60]);
        for i in 0..60 {
            (*shift_sum)[i] += day.shift[i]
        }
    }
    let (id, (minute, _)) = counter
        .iter()
        .map(|(id, shift_sum)| {
            (
                id,
                shift_sum
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, time_asleep)| time_asleep)
                    .unwrap(),
            )
        })
        .max_by_key(|&(_, (_, time_asleep))| time_asleep)
        .unwrap();

    (*id as usize) * minute
}

fn main() {
    println!("{:?}", solve_a(INPUT));
    println!("{:?}", solve_b(INPUT));
}

benchtest! {
    part_a_test: solve_a(INPUT) => 60438,
    part_b_test: solve_b(INPUT) => 47989
}
