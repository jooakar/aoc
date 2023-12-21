use std::{fmt::Display, collections::HashSet};

use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Laser {
    dir: (isize, isize),
    pos: (isize, isize),
}

impl Laser {
    fn next(self) -> Laser {
        Laser{pos: (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1), ..self}
    }
}

fn energized(grid: &[Vec<char>], origin: Laser) -> usize {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let mut lasers = vec![origin];
    let mut visited = HashSet::<Laser>::new();
    while let Some(laser) = lasers.pop() {
        let next = laser.next();
        if visited.contains(&next) { continue; }
        if next.pos.0 >= rows || next.pos.1 >= cols || next.pos.0 < 0 || next.pos.1 < 0 {
            continue;
        }
        visited.insert(next.clone());
        match grid[next.pos.0 as usize][next.pos.1 as usize] {
            '\\' => lasers.push(Laser{dir: (next.dir.1, next.dir.0), ..next}),
            '/' => lasers.push(Laser{dir: (-next.dir.1, -next.dir.0), ..next}),
            '-' => {
                if next.dir.0 != 0 {
                    lasers.push(Laser{dir: (0, 1), ..next});
                    lasers.push(Laser{dir: (0, -1), ..next});
                } else {
                    lasers.push(next);
                }
            },
            '|' => {
                if next.dir.1 != 0 {
                    lasers.push(Laser{dir: (1, 0), ..next});
                    lasers.push(Laser{dir: (-1, 0), ..next});
                } else {
                    lasers.push(next);
                }
            },
            _ => lasers.push(next),
        }
    }
    visited.iter().map(|l| l.pos).unique().count()
}

fn part1(input: String) -> impl Display {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    energized(&grid, Laser{ dir: (0, 1), pos: (0, -1) })
}

fn part2(input: String) -> impl Display {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let row_origins = (0..rows).flat_map(|row| vec![Laser{ dir: (0, 1), pos: (row, -1)}, Laser{ dir: (0, -1), pos: (row, cols)}]);
    let col_origins = (0..cols).flat_map(|col| vec![Laser{ dir: (1, 0), pos: (-1, col)}, Laser{ dir: (-1, 0), pos: (rows, col)}]);
    row_origins.chain(col_origins).map(|l| energized(&grid, l)).max().unwrap()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: String = include_str!("../../input/16.ex").to_string();
        println!("EXAMPLE 1: {}", part1(example.clone()));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: String = include_str!("../../input/16").to_string();
        println!("PART 1: {}", part1(input.clone()));
        println!("PART 2: {}", part2(input));
    }
}
