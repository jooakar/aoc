mod utils;

use itertools::Itertools;
use std::cmp::min;
use utils::nums;

fn part1(input: String) -> String {
    let (seeds, all_maps) = input.split_once("\n\n").unwrap();
    let mut state: Vec<u32> = nums(seeds);

    for maps in all_maps.split("\n\n") {
        let mut next = state.clone();
        for line in maps.lines().skip(1) {
            let map = nums(line);
            let (dest, source, length) = (map[0], map[1], map[2]);
            for (i, s) in state.iter().enumerate() {
                if *s < source || *s - source > length { continue; }
                next[i] = dest + (*s - source);
            }
        }
        state = next;
    }

    state.iter().min().unwrap().to_string()
}

fn part2(input: String) -> String {
    let (seeds, all_maps) = input.split_once("\n\n").unwrap();
    let mut state: Vec<(u64, u64)> = seeds
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<u64>())
        .tuples()
        .map(|(s, l)| (s, s + l))
        .collect();
    for maps in all_maps.split("\n\n") {
        state = state.iter().flat_map(|&(start, end)| {
            let mut mapped = vec![];
            let mut unmapped = vec![(start, end)];
            let mappings = maps.lines().skip(1).map(nums::<u64>).map(|x| (x[0], x[1], x[2]));
            for (dest, source, length) in mappings {
                let mut m = Vec::new();
                for (start, end) in unmapped {
                    let a = (start, end.min(source));
                    let b = (start.max(source), (source + length).min(end));
                    let c = ((source + length).max(start), end);
                    if a.0 < a.1 { m.push(a); }
                    if b.0 < b.1 { mapped.push((b.0 - source + dest, b.1 - source + dest)); }
                    if c.0 < c.1 { m.push(c); }
                }
                unmapped = m;
            }
            mapped.extend(unmapped);
            mapped
        }).collect();
    }

    state.iter().map(|x| x.0).min().unwrap().to_string()
}

fn main() {
    let input: String = include_str!("../../input/05").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
