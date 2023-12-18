use std::fmt::Display;
use std::iter::zip;

use itertools::Itertools;

fn find_match(arrs: &[Vec<bool>], smudges: usize) -> usize {
    for (i, (this, next)) in arrs.iter().tuple_windows().enumerate() {
        let mut maxdiff = smudges;
        let diff = zip(this, next).filter(|(x,y)| x != y).count();
        if diff > maxdiff { continue; }
        if i == 0 && diff == smudges { return i + 1; }
        let mut p1 = i;
        let mut p2 = i + 1;
        loop {
            let diff = zip(&arrs[p1], &arrs[p2]).filter(|(x,y)| x != y).count();
            if diff > maxdiff { break; }
            maxdiff -= diff;
            if p1 == 0 || p2 + 1 == arrs.len() { 
                if maxdiff == 0 { return i + 1; }
                else { break; }
            }
            p1 -= 1;
            p2 += 1;
        }
    }
    0
}

fn solution(input: String, smudges: usize) -> impl Display {
    let mut res = 0;
    for block in input.split("\n\n") {
        let (rows, cols) = (block.lines().count(), block.lines().next().unwrap().len());
        let mut horizontal: Vec<Vec<bool>> = vec![vec![]; rows];
        let mut vertical: Vec<Vec<bool>> = vec![vec![]; cols];
        for (i, line) in block.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let tile = c == '#';
                horizontal[i].push(tile);
                vertical[j].push(tile);
            }
        }
        res += find_match(&horizontal, smudges) * 100;
        res += find_match(&vertical, smudges);
    }

    res
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
    let example: String = include_str!("../../input/13.ex").to_string();
        println!("EXAMPLE 1: {}", solution(example.clone(), 0));
        println!("EXAMPLE 2: {}", solution(example, 1));
    } else {
        let input: String = include_str!("../../input/13").to_string();
        println!("PART 1: {}", solution(input.clone(), 0));
        println!("PART 2: {}", solution(input, 1));
    }
}
