use std::fmt::Display;

use aoc::util::nums::nums;
use itertools::Itertools;

fn part1(input: &str) -> impl Display {
    let (ranges, codes) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|l| nums::<u64>(l).into_iter().collect_tuple().unwrap())
        .collect();
    codes
        .lines()
        .map(|l| nums::<u64>(l)[0])
        .filter(|&c| ranges.iter().any(|r| r.0 <= c && r.1 >= c))
        .count()
}

fn part2(input: &str) -> impl Display {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut merged: Vec<(u64, u64)> = Vec::new();
    ranges
        .lines()
        .map(|l| nums::<u64>(l).into_iter().collect_tuple().unwrap())
        .sorted_by(|(a0, _), (b0, _)| a0.cmp(b0))
        .for_each(|(lo, hi)| {
            if let Some(previous) = merged.last_mut() {
                if lo <= previous.1 {
                    previous.1 = std::cmp::max(previous.1, hi);
                } else {
                    merged.push((lo, hi));
                }
            } else {
                merged.push((lo, hi));
            }
        });
    merged.into_iter().map(|(lo, hi)| hi - lo + 1).sum::<u64>()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/05.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/05");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
