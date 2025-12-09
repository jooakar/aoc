use std::fmt::Display;

use aoc::util::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};

fn part1(input: &str) -> impl Display {
    let mut g = Grid::parse(input);
    let p = find_start(&g);
    traverse(&mut g, p);
    g.bytes.into_iter().filter(|b| *b == b'X').count()
}

fn part2(input: &str) -> impl Display {
    let mut grid = Grid::parse(input);
    let start = find_start(&grid);
    traverse(&mut grid, start);
    let mut counts = Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![0; (grid.width * grid.height) as usize],
    };
    counts[start] = 1;
    for row in 1..grid.height {
        for col in 0..grid.width {
            let p = Point::new(col, row);
            if grid[p] != b'|' {
                continue;
            }
            let mut check_points = vec![];
            for adj in [p + LEFT, p + RIGHT] {
                if grid.contains(adj) && grid[adj] == b'X' {
                    check_points.push(adj + UP);
                }
            }
            if grid[p + UP] == b'|' {
                check_points.push(p + UP);
            }
            let score = check_points.into_iter().map(|cp| counts[cp]).sum::<usize>();
            counts[p] += score.max(1);
        }
        if row == grid.height - 1 {
            return (0..grid.width)
                .map(|col| counts[Point::new(col, row)])
                .sum::<usize>();
        }
    }
    1
}

fn find_start(g: &Grid<u8>) -> Point {
    for i in 0..g.width {
        let p = Point::new(i, 0);
        if g[p] == b'S' {
            return p;
        }
    }
    unreachable!();
}

fn traverse(g: &mut Grid<u8>, pos: Point) {
    g[pos] = b'|';
    let next = pos + DOWN;
    if !g.contains(next) {
        return;
    }
    match g[next] {
        b'.' => traverse(g, next),
        b'^' => {
            g[next] = b'X';
            traverse(g, next + LEFT);
            traverse(g, next + RIGHT);
        }
        _ => return,
    }
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/07.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/07");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
