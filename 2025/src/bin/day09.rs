use std::fmt::Display;

use aoc::util::point::Point;
use itertools::Itertools;

fn part1(input: &str) -> u64 {
    let points: Vec<_> = input.lines().map(|l| parse_point(l)).collect();
    points
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap()
}

fn part2(input: &str) -> impl Display {
    todo!();
    1
}

fn parse_point(data: &str) -> Point {
    let (x, y) = data
        .split_once(',')
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap();
    Point::new(x, y)
}

fn area(p1: Point, p2: Point) -> u64 {
    (p1.x.abs_diff(p2.x) + 1) as u64 * (p1.y.abs_diff(p2.y) + 1) as u64
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/09.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/09");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
