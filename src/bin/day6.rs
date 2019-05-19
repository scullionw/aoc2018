#![feature(test)]

use benchtest::benchtest;
use std::collections::HashMap;
use std::ops::RangeInclusive;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

const INPUT: &str = include_str!("data/input_day6.txt");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
struct Limits {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

impl Coord {
    fn from_str(line: &str) -> Result<Coord> {
        let coords: Vec<_> = line.split(',').map(str::trim).collect();
        Ok(Coord {
            x: coords[0].parse()?,
            y: coords[1].parse()?,
        })
    }

    fn dist_to(&self, x: u32, y: u32) -> u32 {
        let x_dist = if self.x > x { self.x - x } else { x - self.x };
        let y_dist = if self.y > y { self.y - y } else { y - self.y };
        x_dist + y_dist
    }
}

impl Limits {
    fn from_coords(coords: &[Coord]) -> Limits {
        let mut left = std::u32::MAX;
        let mut right = std::u32::MIN;
        let mut top = std::u32::MAX;
        let mut bottom = std::u32::MIN;
        for coord in coords {
            // Adjust left-right limits. X-axis
            if coord.x < left {
                left = coord.x
            } else if coord.x > right {
                right = coord.x
            }
            // Adjust top-bottom limits. Y-axis
            if coord.y < top {
                top = coord.y
            } else if coord.y > bottom {
                bottom = coord.y
            }
        }
        Limits {
            left,
            right,
            top,
            bottom,
        }
    }
    fn rows(&self) -> RangeInclusive<u32> {
        self.top..=self.bottom
    }

    fn columns(&self) -> RangeInclusive<u32> {
        self.left..=self.right
    }

    // Infinite territories are owned by coords on edges of zone.
    // Because moving further than these limits
    // brings you further from all dangerous zones.
    // So owner does not change.
    fn finite_territory(&self, coord: &Coord) -> bool {
        coord.x != self.left
            && coord.x != self.right
            && coord.y != self.top
            && coord.y != self.bottom
    }
}

fn solve_a(seq: &str) -> Result<u32> {
    let coords = seq
        .trim()
        .lines()
        .map(Coord::from_str)
        .collect::<Result<Vec<_>>>()?;

    let limits = Limits::from_coords(&coords);
    let mut closest_to = HashMap::new();

    for x in limits.columns() {
        for y in limits.rows() {
            let mut min_a = std::u32::MAX;
            let mut min_b = std::u32::MAX;
            let mut min_coord = None;
            for coord in &coords {
                let dist = coord.dist_to(x, y);
                if dist < min_a {
                    min_b = min_a;
                    min_a = dist;
                    min_coord = Some(coord);
                } else if dist < min_b {
                    min_b = dist;
                }
            }
            // Don't count as owned land
            // when tied in distance
            if min_a != min_b {
                *closest_to
                    // There will always be a minimum if the list
                    // is not empty.
                    .entry(min_coord.expect("Minimum not found."))
                    .or_insert(0) += 1
            }
        }
    }
    closest_to.retain(|coord, _| limits.finite_territory(coord));
    Ok(*closest_to
        .iter()
        .max_by_key(|&(_, owned_count)| owned_count)
        // If this happens there would be no answer.
        .expect("There is no owned land.")
        .1)
}

fn solve_b(seq: &str) -> Result<u32> {
    let coords = seq
        .trim()
        .lines()
        .map(Coord::from_str)
        .collect::<Result<Vec<_>>>()?;

    let limits = Limits::from_coords(&coords);

    let mut safe_count = 0;
    for x in limits.columns() {
        for y in limits.rows() {
            let total_dist = coords.iter().map(|coord| coord.dist_to(x, y)).sum::<u32>();
            if total_dist < 10_000 {
                safe_count += 1;
            }
        }
    }
    Ok(safe_count)
}

fn main() -> Result<()> {
    println!("{:?}", solve_a(INPUT)?);
    println!("{:?}", solve_b(INPUT)?);
    Ok(())
}

benchtest! {
    part_a_test: solve_a(INPUT).unwrap() => 3620,
    part_b_test: solve_b(INPUT).unwrap() => 39930
}
