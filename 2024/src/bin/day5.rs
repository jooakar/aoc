use std::{cmp::Ordering::*, fmt::Display};

use aoc::util::nums::nums;

fn part1(input: &str) -> impl Display {
    let mut map = [[Greater; 100]; 100];
    let (rules, updates) = input.split_once("\n\n").unwrap();
    rules
        .lines()
        .map(|line| nums::<usize>(line))
        .for_each(|nums| map[nums[0]][nums[1]] = Less);
    updates
        .lines()
        .map(nums::<usize>)
        .filter(|update| {
            update
                .iter()
                .is_sorted_by(|from, to| map[**from][**to] == Less)
        })
        .map(|update| update[update.len() / 2])
        .sum::<usize>()
}

fn part2(input: &str) -> impl Display {
    let mut map = [[Greater; 100]; 100];
    let (rules, updates) = input.split_once("\n\n").unwrap();
    rules
        .lines()
        .map(|line| nums::<usize>(line))
        .for_each(|nums| map[nums[0]][nums[1]] = Less);
    updates
        .lines()
        .map(nums::<usize>)
        .filter(|update| {
            !update
                .iter()
                .is_sorted_by(|from, to| map[**from][**to] == Less)
        })
        .map(|mut update| {
            let middle = update.len() / 2;
            update.select_nth_unstable_by(middle, |from, to| map[*from][*to]);
            update[middle]
        })
        .sum::<usize>()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/5.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/5");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
