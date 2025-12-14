use aoc::util::nums::nums;
use itertools::Itertools;
use regex::Regex;

fn part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines
        .into_iter()
        .map(|m| {
            let sets: Vec<_> = m.buttons.into_iter().powerset().collect();
            sets.into_iter()
                .filter(|s| {
                    let mut lights = vec![false; m.lights.len()];
                    s.iter()
                        .for_each(|b| b.iter().for_each(|i| lights[*i] = !lights[*i]));
                    lights == m.lights
                })
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    todo!();
    1
}

fn parse_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"\[([.#]+)\]([^{]+)\{([0-9,]+)\}").unwrap();
    let re2 = Regex::new(r"\([0-9,]+\)").unwrap();
    re.captures_iter(input)
        .map(|a| a.extract())
        .map(|(_, [a, b, c])| {
            let lights = a.chars().map(|c| c == '#').collect();
            let buttons = re2.find_iter(b.trim()).map(|i| nums(i.as_str())).collect();
            let joltages = nums(c);
            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/10.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/10");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
