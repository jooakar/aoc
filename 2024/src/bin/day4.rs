use std::fmt::Display;

use aoc::util::grid::*;
use aoc::util::point::*;

fn part1(input: &str) -> impl Display {
    let grid = Grid::parse(input);
    let mut result = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let p = Point::new(x, y);

            if grid[p] == b'X' {
                for step in DIAGONAL {
                    result += (grid.contains(p + step * 3)
                        && grid[p + step] == b'M'
                        && grid[p + step * 2] == b'A'
                        && grid[p + step * 3] == b'S') as u32;
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> impl Display {
    let grid = Grid::parse(input);
    let mut result = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let p = Point::new(x, y);

            if grid[p] == b'A' {
                let steps = [Point::new(1, 1), Point::new(1, -1)];
                result += steps.iter().all(|&step| {
                    grid.contains(p + step)
                        && grid.contains(p + step * -1)
                        && grid[p + step].abs_diff(grid[p + step * -1]) == 6
                }) as u32;
            }
        }
    }
    result
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example = include_str!("../../input/4.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input = include_str!("../../input/4");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
