use std::fmt::Display;

use aoc::util::{grid::*, point::*};

fn solution(input: &str, p2: bool) -> impl Display {
    let grid = Grid::parse(input);
    let mut res = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            if grid[p] == b'0' {
                let visited = &mut vec![];
                score(&grid, p, visited, p2);
                res += visited.len();
            }
        }
    }
    res
}

fn score(grid: &Grid<u8>, start: Point, visited: &mut Vec<Point>, p2: bool) {
    ORTHOGONAL.into_iter().for_each(|d| {
        let next = start + d;
        if !grid.contains(next) || grid[next].checked_sub(grid[start]) != Some(1) {
            return;
        }
        if grid[next] == b'9' {
            if p2 || !visited.contains(&next) {
                visited.push(next);
            }
        } else {
            score(grid, next, visited, p2);
        }
    });
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/10.ex");
        println!("EXAMPLE 1: {}", solution(example, false));
        println!("EXAMPLE 2: {}", solution(example, true));
    } else {
        let input: &str = include_str!("../../input/10");
        println!("PART 1: {}", solution(input, false));
        println!("PART 2: {}", solution(input, true));
    }
}
