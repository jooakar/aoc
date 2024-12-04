use std::fmt::Display;

fn part1(input: &str) -> impl Display {
    1234
}

fn part2(input: &str) -> impl Display {
    1234
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/DAYNUM.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/DAYNUM");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
