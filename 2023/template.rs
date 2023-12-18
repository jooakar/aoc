use std::fmt::Display;

fn part1(input: String) -> impl Display {
    todo!();
    0
}

fn part2(input: String) -> impl Display {
    todo!();
    0
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: String = include_str!("../../input/DAYNUM.ex").to_string();
        println!("EXAMPLE 1: {}", part1(example.clone()));
        println!("EXAMPLE 2: {}", part1(example));
    } else {
        let input: String = include_str!("../../input/DAYNUM").to_string();
        println!("PART 1: {}", part1(input.clone()));
        println!("PART 2: {}", part1(input));
    }
}
