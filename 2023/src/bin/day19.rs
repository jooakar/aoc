use std::{fmt::Display, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
struct Operation {
    field: char,
    op: char,
    cmp: usize,
    result: String,
}

impl Operation {
    fn from(s: &str, res: &str) -> Operation {
        let mut iter = s.chars();
        Operation{
            field: iter.next().unwrap(),
            op: iter.next().unwrap(),
            cmp: iter.collect::<String>().parse().unwrap(),
            result: res.to_string(),
        }
    }

    fn always(res: &str) -> Operation {
        Operation {field: '-', op: '-', cmp: 0, result: res.to_string()}
    }

    fn resolve_part(&self, target: &HashMap<char, usize>) -> Option<&str> {
        if self.field == '-' { return Some(&self.result) }
        let val = target.get(&self.field).unwrap();
        let pass = match self.op {
            '>' => self.cmp < *val,
            '<' => self.cmp > *val,
            _ => false,
        };
        if pass { Some(&self.result) } else { None }
    }
}

type WorkFlow<'a> = HashMap<&'a str, Vec<Operation>>;

fn part1(input: String) -> impl Display {
    let (flows, parts) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"([a-z])=(\d+)").unwrap();
    let parts = parts.lines()
        .map(|line| { 
            re.captures_iter(line)
                .map(|m| (m[1].chars().next().unwrap(), m[2].parse().unwrap()))
                .collect::<HashMap<char, usize>>()
        })
        .collect::<Vec<_>>();
    let mut wf = WorkFlow::new();
    for flow in flows.lines() {
        let (key, rules) = flow.split_once('{').unwrap();
        let mut instructions = vec![];
        for rule in rules.trim_end_matches('}').split(',') {
            let ins = match rule.split_once(':') {
                Some(m) => Operation::from(m.0, m.1),
                None => Operation::always(rule),
            };
            instructions.push(ins);
        }
        wf.insert(key, instructions);
    }
    let mut ans = 0;
    for part in parts {
        let mut flow = wf.get("in").unwrap();
        loop {
            let next = flow.iter().find_map(|op| op.resolve_part(&part)).unwrap();
            match next {
                "A" => { ans += part.values().sum::<usize>(); break; },
                "R" => { break; },
                other => flow = wf.get(other).unwrap(),
            }
        }
    }
    ans
}


type Parts = HashMap<char, (usize, usize)>;

fn find_path(wf: &WorkFlow, parts: Parts) -> Option<Parts> {
    Some(parts)
}

fn part2(input: String) -> impl Display {
    let (flows, _) = input.split_once("\n\n").unwrap();
    let mut wf = WorkFlow::new();
    for flow in flows.lines() {
        let (key, rules) = flow.split_once('{').unwrap();
        let mut instructions = vec![];
        for rule in rules.trim_end_matches('}').split(',') {
            let ins = match rule.split_once(':') {
                Some(m) => Operation::from(m.0, m.1),
                None => Operation::always(rule),
            };
            instructions.push(ins);
        }
        wf.insert(key, instructions);
    }
    let parts = HashMap::from([('x', (1, 4000)), ('m', (1, 4000)), ('a', (1, 4000)), ('s', (1, 4000))]);
    let mut accepted: Vec<Parts> = vec![];

    // TODO: Find the actual path, does nothing right now
    
    let mut res = 1;
    accepted.push(parts);
    for c in "xmas".chars() {
        let (min, max) = accepted.iter()
            .map(|a| a.get(&c).unwrap())
            .fold((4000,1), |acc, range| (acc.0.min(range.0), acc.1.max(range.1)));
        if min >= max { continue; }
        res *= max - min;
    }
    res
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: String = include_str!("../../input/19.ex").to_string();
        println!("EXAMPLE 1: {}", part1(example.clone()));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: String = include_str!("../../input/19").to_string();
        println!("PART 1: {}", part1(input.clone()));
        println!("PART 2: {}", part2(input));
    }
}
