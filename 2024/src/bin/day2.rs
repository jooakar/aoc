use std::fmt::Display;

use aoc::util::nums::nums;
use itertools::Itertools;

fn valid(input: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = input.iter().tuple_windows().map(|(a, b)| a - b).collect();
    let negatives = diffs.iter().filter(|x| **x < 0).count();
    diffs.iter().all(|x| x.abs() <= 3 && x.abs() >= 1)
        && (negatives >= diffs.len() || negatives == 0)
}

fn part1(input: &str) -> impl Display {
    input.lines().map(nums::<i64>).filter(|r| valid(r)).count()
}

fn part2(input: &str) -> impl Display {
    input
        .lines()
        .map(nums::<i64>)
        .filter(|r| {
            if valid(r) {
                return true;
            }
            (0..r.len()).any(|i| {
                let concatenated = r[..i].iter().chain(&r[i + 1..]).cloned().collect_vec();
                valid(&concatenated)
            })
        })
        .count()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/2.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/2");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
