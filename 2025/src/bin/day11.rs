use std::collections::HashMap;

fn solution(input: &str, start: &str) -> (usize, usize) {
    let keys: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(k, vs)| (k, vs.split_whitespace().collect()))
        .collect();
    let mut current: HashMap<_, _> = HashMap::from([(start, (1, 0, 0, 0))]);
    let mut score = (0, 0);
    loop {
        let mut new: HashMap<&str, (usize, usize, usize, usize)> = HashMap::new();
        for (k, (all, fft, dac, valid)) in current {
            if k == "out" {
                score.0 += all;
                score.1 += valid;
                continue;
            }
            let vs = keys.get(k).unwrap();
            for v in vs {
                let entry = new.entry(v).or_insert((0, 0, 0, 0));
                entry.0 += all;
                entry.1 += if k == "fft" { all } else { fft };
                entry.2 += if k == "dac" { all } else { dac };
                entry.3 += match (k, fft > 0, dac > 0) {
                    ("fft", _, true) => dac,
                    ("dac", true, _) => fft,
                    _ => valid,
                };
            }
        }
        if new.is_empty() {
            break;
        }
        current = new;
    }
    score
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/11.ex");
        // different examples for parts 1 and 2, can't use the same file
        // println!("EXAMPLE 1: {}", solution(example, "you").0);
        println!("EXAMPLE 2: {}", solution(example, "svr").1);
    } else {
        let input: &str = include_str!("../../input/11");
        println!("PART 1: {}", solution(input, "you").0);
        println!("PART 2: {}", solution(input, "svr").1);
    }
}
