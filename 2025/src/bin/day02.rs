use itertools::Itertools;
use std::fmt::Display;

use aoc::util::nums::nums;

fn part1(input: &str) -> impl Display {
    let mut score = 0;
    input
        .split(",")
        .map(|s| nums::<u64>(s).into_iter().collect_tuple().unwrap())
        .for_each(|(a, b)| {
            for i in a..=b {
                let n = i.ilog10() + 1;
                if n % 2 != 0 {
                    continue;
                }
                let n2 = 10_u64.pow(n / 2);
                if i % n2 == i / n2 {
                    score += i;
                }
            }
        });
    score
}

fn part2(input: &str) -> impl Display {
    let mut score = 0;
    input
        .split(",")
        .map(|s| nums::<u64>(s).into_iter().collect_tuple().unwrap())
        .for_each(|(a, b)| {
            for i in a..=b {
                // very bad but it works
                let s = i.to_string();
                for j in 1..(s.len()) {
                    if s.len() % j != 0 {
                        continue;
                    }
                    let start = &s[0..j];
                    if s.as_bytes()
                        .chunks(j)
                        .map(|chunk| std::str::from_utf8(chunk).unwrap())
                        .all(|c| c == start)
                    {
                        score += i;
                        break;
                    }
                }
            }
        });
    score
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/02.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/02");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
