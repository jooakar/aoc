use std::{fmt::Display, collections::HashMap};

fn tilt(grid: &mut Vec<Vec<char>>) {
    let (rows, cols) = (grid.len(), grid[0].len());
    for col in 0..cols {
        let mut pos = 0;
        for row in 0..rows {
            match grid[row][col] {
                '#' => pos = row + 1,
                'O' => {
                    grid[row][col] = '.';
                    grid[pos][col] = 'O';
                    pos += 1;
                },
                _ => ()
            }
        }
    }
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let n = grid.len();
    grid.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .filter(|x| **x == 'O')
                .count() * (n - row)
        })
        .sum::<usize>()
}

fn part1(input: String) -> impl Display {
    let mut grid = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    tilt(&mut grid);
    score(&grid)
}

fn rotate(grid: &mut Vec<Vec<char>>) {
    grid.reverse();
    for i in 1..grid.len() {
        let (left, right) = grid.split_at_mut(i);
        for (j, left_item) in left.iter_mut().enumerate().take(i) {
            std::mem::swap(&mut left_item[i], &mut right[0][j]);
        }
    }
    
}

fn part2(input: String) -> impl Display {
    let mut grid = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut results = HashMap::<Vec<Vec<char>>, Vec<usize>>::new();
    let mut i = 1;
    while i <= 1000000000 {
        for _ in 0..4 {
            tilt(&mut grid);
            rotate(&mut grid);
        }
        let res = results.entry(grid.clone()).or_default();
        if res.len() >= 3 && i <= 1000 {
            let cycle = res[2] - res[1];
            i += cycle * ((1000000000-i) / cycle);
        }
        res.push(i);
        i += 1;
    }
    score(&grid)
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: String = include_str!("../../input/14.ex").to_string();
        println!("EXAMPLE 1: {}", part1(example.clone()));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: String = include_str!("../../input/14").to_string();
        println!("PART 1: {}", part1(input.clone()));
        println!("PART 2: {}", part2(input));
    }
}
