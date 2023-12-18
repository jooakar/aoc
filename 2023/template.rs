use std::fmt::Display;

fn part1(input: String) -> impl Display {
    todo!()
}

fn part2(input: String) -> impl Display {
    todo!()
}

fn main() {
    let example: String = include_str!("../../input/DAYNUM.ex").to_string();
    let input: String = include_str!("../../input/DAYNUM").to_string();
    println!("EXAMPLE 1: {}", part1(example.clone()));
    println!("PART 1: {}", part1(input.clone()));
    println!("EXAMPLE 2: {}", part2(example));
    println!("PART 2: {}", part2(input));
}
