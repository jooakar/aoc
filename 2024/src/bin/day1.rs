use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

fn part1(input: String) -> impl Display {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|s| s.parse::<u32>())
                .next_tuple()
                .unwrap()
        })
        .unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| r.abs_diff(l))
        .sum::<u32>()
}

fn part2(input: String) -> impl Display {
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (l, r) = line
                .split_ascii_whitespace()
                .flat_map(|s| s.parse::<u32>())
                .next_tuple()
                .unwrap();
            *left.entry(l).or_insert(0) += 1;
            *right.entry(r).or_insert(0) += 1;
        });
    left.into_iter()
        .map(|(k, v)| k * v * right.get(&k).unwrap_or(&0))
        .sum::<u32>()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: String = include_str!("../../input/1.ex").to_string();
        println!("EXAMPLE 1: {}", part1(example.clone()));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: String = include_str!("../../input/1").to_string();
        println!("PART 1: {}", part1(input.clone()));
        println!("PART 2: {}", part2(input));
    }
}
