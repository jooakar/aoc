use std::fmt::Display;

use aoc::util::nums::nums;

fn part1(input: &str) -> impl Display {
    let lines: Vec<&str> = input.lines().collect();
    let rows: Vec<Vec<usize>> = lines[0..(lines.len() - 1)]
        .iter()
        .map(|l| nums::<usize>(l))
        .collect();
    lines
        .into_iter()
        .last()
        .map(|o| {
            o.split_whitespace()
                .enumerate()
                .map(|(i, op)| match op {
                    "+" => rows.iter().map(|r| r[i]).sum::<usize>(),
                    "*" => rows.iter().map(|r| r[i]).product::<usize>(),
                    _ => unreachable!(),
                })
                .sum::<usize>()
        })
        .unwrap()
}

fn part2(input: &str) -> impl Display {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (ops, rows) = lines.split_last().unwrap();
    let mut op: fn(&[u64]) -> u64 = |_| unreachable!();
    let mut nums = Vec::new();
    let mut score = 0;
    for (i, &c) in ops.iter().enumerate() {
        op = match c {
            '+' => |nums| nums.iter().sum(),
            '*' => |nums| nums.iter().product(),
            _ => op,
        };
        let num = rows
            .iter()
            .filter_map(|r| r[i].to_digit(10).map(|d| d as u64))
            .fold(0, |acc, d| acc * 10 + d);
        if num == 0 || i == ops.len() - 1 {
            if num != 0 && i == ops.len() - 1 {
                nums.push(num);
            }
            score += op(&nums);
            nums.clear();
        } else {
            nums.push(num);
        }
    }
    score
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/06.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/06");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
