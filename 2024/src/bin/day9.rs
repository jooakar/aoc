use std::fmt::Display;

use itertools::{Either, Itertools};

fn part1(input: &str) -> impl Display {
    let nums: Vec<_> = input
        .chars()
        .map(|n| n.to_digit(10).unwrap_or(0) as usize)
        .collect();
    let mut blocks = Vec::with_capacity(nums.iter().sum());
    for (i, n) in nums.into_iter().enumerate() {
        let c = if i % 2 == 0 { Some(i / 2) } else { None };
        for _ in i..(i + n) {
            blocks.push(c);
        }
    }
    let mut i = 0;
    let mut j = blocks.len() - 1;
    let mut res = 0;
    while i < blocks.len() {
        if blocks[j] == None {
            j -= 1;
        } else if blocks[i] == None {
            if j > i {
                res += blocks[j].unwrap() * i;
                blocks[j] = None;
                j -= 1;
            }
            i += 1;
        } else {
            res += blocks[i].unwrap() * i;
            i += 1;
        }
    }
    res
}

fn part2(input: &str) -> impl Display {
    let (mut files, mut blanks): (Vec<_>, Vec<_>) = input
        .chars()
        .map(|n| n.to_digit(10).unwrap_or(0) as usize)
        .enumerate()
        .partition_map(|(i, n)| match i % 2 {
            0 => Either::Left((n, true)),
            1 => Either::Right(n),
            _ => unreachable!(),
        });
    let mut blocks: Vec<Option<usize>> =
        Vec::with_capacity(files.iter().map(|b| b.0).sum::<usize>() + blanks.iter().sum::<usize>());
    let mut i = 0;
    while i < files.len() {
        for _ in 0..files[i].0 {
            blocks.push(if files[i].1 { Some(i) } else { None });
        }
        for j in (i + 1..files.len()).rev() {
            if blanks[i] <= 0 {
                break;
            }
            if files[j].1 && files[j].0 <= blanks[i] {
                blanks[i] -= files[j].0;
                for _ in 0..files[j].0 {
                    blocks.push(Some(j));
                }
                files[j].1 = false;
            }
        }
        for _ in 0..(blanks[i]) {
            blocks.push(None);
        }
        i += 1;
    }
    blocks
        .into_iter()
        .enumerate()
        .map(|(i, v)| i * v.unwrap_or(0))
        .sum::<usize>()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/9.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/9");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
