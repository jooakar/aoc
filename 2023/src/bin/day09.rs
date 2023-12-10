mod utils;
use itertools::Itertools;
use utils::nums;

fn part1(input: String) -> String {
    input.lines()
        .map(nums::<i64>)
        .map(find_next)
        .sum::<i64>()
        .to_string()
}

fn part2(input: String) -> String {
    input.lines()
        .map(|x| nums::<i64>(x).into_iter().rev().collect())
        .map(find_next)
        .sum::<i64>()
        .to_string()
}

fn find_next(pattern: Vec<i64>) -> i64 {
    if pattern.iter().all(|&x| x == 0) { return 0; }
    let next = pattern.iter().tuple_windows().map(|(a,b)| b - a).collect();
    pattern.last().unwrap() + find_next(next)
}

fn main() {
    let input: String = include_str!("../../input/09").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
