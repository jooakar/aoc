use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

fn part1(input: &str) -> impl Display {
    let points: Vec<_> = input.lines().map(|l| Point::parse(l)).collect();
    let mut distances: Vec<(i64, usize, usize)> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            distances.push((points[i].dist(&points[j]), i, j))
        }
    }
    distances.sort_by(|a, b| a.0.cmp(&b.0));
    let mut circuits: Vec<HashSet<usize>> = vec![];
    for (_, i, j) in distances.iter().take(1000) {
        let a = circuits.iter().position(|c| c.contains(i));
        let b = circuits.iter().position(|c| c.contains(j));
        match (a, b) {
            (Some(ai), Some(bi)) => {
                if ai != bi {
                    circuits[ai] = circuits[ai].union(&circuits[bi]).copied().collect();
                    circuits.swap_remove(bi);
                }
            }
            (None, Some(bi)) => {
                circuits[bi].insert(*i);
            }
            (Some(ai), None) => {
                circuits[ai].insert(*j);
            }
            (None, None) => circuits.push(HashSet::from_iter([*i, *j])),
        };
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits
        .into_iter()
        .take(3)
        .map(|c| c.len())
        .product::<usize>()
}

fn part2(input: &str) -> impl Display {
    let points: Vec<_> = input.lines().map(|l| Point::parse(l)).collect();
    let mut distances: Vec<(i64, usize, usize)> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            distances.push((points[i].dist(&points[j]), i, j))
        }
    }
    distances.sort_by(|a, b| a.0.cmp(&b.0));
    let mut circuits: Vec<HashSet<usize>> = vec![];
    for (_, i, j) in distances.iter() {
        let a = circuits.iter().position(|c| c.contains(i));
        let b = circuits.iter().position(|c| c.contains(j));
        match (a, b) {
            (Some(ai), Some(bi)) => {
                if ai != bi {
                    circuits[ai] = circuits[ai].union(&circuits[bi]).copied().collect();
                    circuits.swap_remove(bi);
                }
            }
            (None, Some(bi)) => {
                circuits[bi].insert(*i);
            }
            (Some(ai), None) => {
                circuits[ai].insert(*j);
            }
            (None, None) => circuits.push(HashSet::from_iter([*i, *j])),
        };
        if circuits.iter().any(|c| c.len() == points.len()) {
            return points[*i].x * points[*j].x;
        }
    }
    unreachable!();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn parse(data: &str) -> Self {
        let (x, y, z) = data
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, y, z }
    }

    fn dist(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/08.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/08");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
