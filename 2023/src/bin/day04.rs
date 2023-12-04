use std::collections::HashSet;

use itertools::Itertools;

mod utils;

fn part1(input: String) -> String {
    let mut res = 0;
    for line in input.lines() {
        let (_, data) = line.split_once(": ").unwrap();
        let (win, guess): (HashSet<u32>, HashSet<u32>) = data
            .split('|')
            .map(|row| utils::nums(row).iter().cloned().collect())
            .collect_tuple()
            .unwrap();
        let wins = win.intersection(&guess).count();
        if wins > 0  { res += 1 << (wins - 1)}
    }
    res.to_string()
}

fn part2(input: String) -> String {
    let mut counts = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (_, data) = line.split_once(": ").unwrap();
        let (win, guess): (HashSet<u32>, HashSet<u32>) = data
            .split('|')
            .map(|row| utils::nums(row).iter().cloned().collect())
            .collect_tuple()
            .unwrap();
        for j in 1..=win.intersection(&guess).count() {
            counts[i + j] += counts[i];
        }
    }
    counts.iter().sum::<u32>().to_string()
}

fn main() {
    let input: String = include_str!("../../input/04").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
