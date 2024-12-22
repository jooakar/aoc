use std::fmt::Display;

use aoc::util::nums::nums;

fn solution(input: &str, p2: bool) -> impl Display {
    input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(k, v)| (k.parse().unwrap(), nums::<usize>(v)))
        .filter(|(target, parts)| is_possible(*target, 0, &parts[..], p2))
        .map(|(k, _)| k)
        .sum::<usize>()
}

fn is_possible(target: usize, carry: usize, parts: &[usize], p2: bool) -> bool {
    if carry > target {
        return false;
    }
    if parts.len() == 0 {
        return target == carry;
    }
    let mut con = carry;
    let mut post = parts[0];
    while post > 0 {
        con *= 10;
        post /= 10;
    }
    con += parts[0];
    return is_possible(target, carry + parts[0], &parts[1..], p2)
        || is_possible(target, carry * parts[0], &parts[1..], p2)
        || (p2 && is_possible(target, con, &parts[1..], p2));
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/7.ex");
        println!("EXAMPLE 1: {}", solution(example, false));
        println!("EXAMPLE 2: {}", solution(example, true));
    } else {
        let input: &str = include_str!("../../input/7");
        println!("PART 1: {}", solution(input, false));
        println!("PART 2: {}", solution(input, true));
    }
}
