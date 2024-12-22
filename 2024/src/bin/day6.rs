use std::fmt::Display;

use aoc::util::{grid::*, point::*};

fn part1(input: &str) -> impl Display {
    let grid = Grid::parse(input);
    let mut dir = UP;
    let mut pos = grid.find_point(b'^').unwrap();
    let mut res = 0;
    let mut visited: Vec<Point> = vec![];
    loop {
        res += if visited.contains(&pos) { 0 } else { 1 };
        visited.push(pos);
        let next = pos + dir;
        if !grid.contains(next) {
            return res;
        }
        if grid[next] == b'#' {
            dir = Point::new(-dir.y, dir.x);
        };
        pos += dir;
    }
}

fn part2(input: &str) -> impl Display {
    todo!();
    1
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/6.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/6");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
