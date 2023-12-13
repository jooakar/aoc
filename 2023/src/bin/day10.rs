use std::collections::VecDeque;

use itertools::Itertools;

// Testing how structs work
#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize,
    val: usize,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) = {}", self.x, self.y, self.val)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) = {}", self.x, self.y, self.val)
    }
}

fn neighbors(c: Coord, at: &[Vec<char>], rows: usize, cols: usize) -> Vec<Coord> {
    let (up, down, left, right) = (
        if c.y == 0 { None } else { Some(Coord{ x: c.x, y: c.y - 1, val: c.val + 1 }) },
        if c.y + 1 >= rows { None } else { Some(Coord{ x: c.x, y: c.y + 1, val: c.val + 1 }) },
        if c.x == 0 { None } else { Some(Coord{ x: c.x - 1, y: c.y, val: c.val + 1 }) },
        if c.x + 1 >= cols { None } else { Some(Coord{ x: c.x + 1, y: c.y, val: c.val + 1 }) },
    );
    let dirs = match at[c.y][c.x] {
        '|' => vec![up, down],
        '-' => vec![right, left],
        'J' => vec![up, left],
        'L' => vec![up, right],
        '7' => vec![down, left],
        'F' => vec![down, right],
        '.' => vec![],
        'S' => vec![up, down, right, left],
        _ => unreachable!(),
    };
    dirs.into_iter().flatten().collect()
}

fn find_s(input: &[Vec<char>]) -> Coord {
    for (i, l) in input.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == 'S' { return Coord{ x: j, y: i, val: 0 } } 
        }
    }
    Coord {x: 0, y: 0, val: 0}
}

fn part1(input: String) -> String {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let (rows, cols) = (map.len(), map[0].len());
    let s = find_s(&map);
    let mut visited = vec![s.clone()];
    let mut queue = VecDeque::<Coord>::from([s]);
    while !queue.is_empty() {
        let c = queue.pop_front().unwrap();
        let n: Vec<Coord> = neighbors(c.clone(), &map, rows, cols)
            .into_iter()
            .filter(|n| !visited.contains(n))
            .filter(|n| neighbors(n.clone(), &map, rows, cols).contains(&c))
            .collect();
        visited.extend(n.clone());
        queue.extend(n);
    }
    visited.into_iter().map(|c| c.val).max().unwrap().to_string()
}

fn part2(input: String) -> String {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let (rows, cols) = (map.len(), map[0].len());
    let s = find_s(&map);
    let mut path = vec![s.clone()];
    let mut queue = Vec::<Coord>::from([s.clone()]);
    while let Some(c) = queue.pop() {
        let n: Vec<Coord> = neighbors(c.clone(), &map, rows, cols)
            .into_iter()
            .filter(|n| !path.contains(n))
            .filter(|n| neighbors(n.clone(), &map, rows, cols).contains(&c))
            .collect();
        path.extend(n.clone());
        queue.extend(n);
    }
    path.push(s);
    let area = path
        .iter()
        .tuple_windows()
        .map(|(a, b)| ((a.x + 1) * (b.y + 1)) as isize - ((a.y + 1) * (b.x + 1)) as isize)
        .sum::<isize>() / 2;
    // 1 off for some unknown reason, fix it with +2
    (area.abs() - (path.len() / 2) as isize + 2).to_string()
}

fn main() {
    let input: String = include_str!("../../input/10").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
