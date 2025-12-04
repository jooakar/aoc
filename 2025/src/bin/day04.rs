use std::fmt::Display;

use aoc::util::{
    grid::Grid,
    point::{Point, DIAGONAL},
};

fn part1(input: &str) -> impl Display {
    let g = Grid::parse(input);
    let mut score = 0;
    for x in 0..g.height {
        for y in 0..g.width {
            let p = Point::new(x, y);
            if g[p] != b'@' {
                continue;
            }
            if DIAGONAL
                .iter()
                .map(|&dir| p + dir)
                .filter(|&dp| g.contains(dp) && g[dp] == b'@')
                .count()
                < 4
            {
                score += 1;
            }
        }
    }
    score
}

fn part2(input: &str) -> impl Display {
    let mut g = Grid::parse(input);
    let mut temp = 0;
    let mut score = 0;
    // shitty brute force solution :D
    loop {
        for x in 0..g.height {
            for y in 0..g.width {
                let p = Point::new(x, y);
                if g[p] != b'@' {
                    continue;
                }
                if DIAGONAL
                    .iter()
                    .map(|&dir| p + dir)
                    .filter(|&dp| g.contains(dp) && g[dp] == b'@')
                    .count()
                    < 4
                {
                    temp += 1;
                    g[p] = b'.';
                }
            }
        }
        score += temp;
        if temp <= 0 {
            break;
        }
        temp = 0;
    }
    score
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/04.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/04");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
