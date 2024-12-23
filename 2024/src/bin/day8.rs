use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::util::{grid::*, point::*};
use itertools::Itertools;

fn solution(input: &str, p2: bool) -> impl Display {
    let grid = Grid::parse(input);
    let mut antennas: HashMap<u8, Vec<Point>> = HashMap::new();
    let mut antipoints: HashSet<Point> = HashSet::new();
    for i in 0..grid.width {
        for j in 0..grid.height {
            let p = Point::new(i, j);
            if grid[p] != b'.' {
                antennas.entry(grid[p]).or_insert(vec![]).push(p);
            }
        }
    }
    for (_, points) in antennas.iter() {
        points.iter().tuple_combinations().for_each(|(&a, &b)| {
            let diff = a - b;
            let start = if p2 { 0 } else { 1 };
            for i in start.. {
                let d1 = b - diff * i;
                let d2 = a + diff * i;
                if grid.contains(d1) {
                    antipoints.insert(d1);
                }
                if grid.contains(d2) {
                    antipoints.insert(d2);
                }
                if !p2 || !(grid.contains(d1) || grid.contains(d2)) {
                    break;
                }
            }
        })
    }
    antipoints.len()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/8.ex");
        println!("EXAMPLE 1: {}", solution(example, false));
        println!("EXAMPLE 2: {}", solution(example, true));
    } else {
        let input: &str = include_str!("../../input/8");
        println!("PART 1: {}", solution(input, false));
        println!("PART 2: {}", solution(input, true));
    }
}
