use std::collections::HashMap;

use num::Integer;

fn part1(input: String) -> String {
    let mut paths: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current: &str = "AAA";
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();
    for line in input.lines().skip(2) {
        let (source, options) = line.split_once(" = ").unwrap();
        let parens: &[char] = &['(', ')'];
        let options = options
            .trim_matches(parens)
            .split_once(", ")
            .unwrap();
        paths.insert(source, options);
    }
    let mut res = 0;
    let n = instructions.len();
    while current != "ZZZ" {
        current = match instructions[res % n] {
            'L' => paths[current].0,
            'R' => paths[current].1,
            _ => unreachable!(),
        };
        res += 1;
    }

    res.to_string()
}

fn part2(input: String) -> String {
    let mut paths: HashMap<&str, (&str, &str)> = HashMap::new();
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();
    for line in input.lines().skip(2) {
        let (source, options) = line.split_once(" = ").unwrap();
        let parens: &[char] = &['(', ')'];
        let options = options
            .trim_matches(parens)
            .split_once(", ")
            .unwrap();
        paths.insert(source, options);
    }
    let mut current: Vec<&str> = paths.clone().into_keys().filter(|s| s.ends_with('A')).collect();
    let mut lengths: Vec<usize> = vec![];
    let mut i = 0;
    while !current.is_empty() {
        for (j, node) in current.clone().iter().enumerate() {
            current[j] = match instructions[i % instructions.len()] {
                'L' => paths[node].0,
                'R' => paths[node].1,
                _ => unreachable!(),
            };
            if current[j].ends_with('Z') {
                lengths.push(i + 1);
            }
        }
        current.retain(|s| !s.ends_with('Z'));
        i += 1;
    }

    lengths.into_iter().fold(1, |total, l| l.lcm(&total)).to_string()
}

fn main() {
    let input: String = include_str!("../../input/08").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
