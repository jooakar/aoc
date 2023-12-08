mod utils;
use itertools::Itertools;
use utils::nums;

fn part1(input: String) -> String {
    let (times, distances) = input.split('\n').map(nums::<u32>).next_tuple().unwrap();
    let mut res = 1;
    for (time, distance) in times.iter().zip(distances.iter()).collect::<Vec<_>>() {
        res *= (0..*time).map(|i| (time - i) * i).filter(|i| i > distance).count();
    }
    res.to_string()
}

fn part2(input: String) -> String {
    let (time, distance) = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
        .flat_map(|num| num.parse::<u64>())
        .next_tuple()
        .unwrap();
    (0..time).map(|i| (time - i) * i).filter(|i| i > &distance).count().to_string()
}

fn main() {
    let input: String = include_str!("../../input/06").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
