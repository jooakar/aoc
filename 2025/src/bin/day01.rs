use std::fmt::Display;

fn part1(input: &str) -> impl Display {
    let mut dial = 50;
    let mut zeroes = 0;
    input.lines().map(|l| l.split_at(1)).for_each(|(h, t)| {
        let num = t
            .parse::<i32>()
            .map(|n| if h == "L" { -n } else { n })
            .unwrap();
        dial = (dial + num) % 100;
        zeroes += if dial == 0 { 1 } else { 0 };
    });
    zeroes
}

fn part2(input: &str) -> impl Display {
    let mut dial = 50;
    let mut zeroes = 0;
    input.lines().map(|l| l.split_at(1)).for_each(|(h, t)| {
        let num = t
            .parse::<i32>()
            .map(|n| if h == "L" { -n } else { n })
            .unwrap();
        dial += num;
        while dial >= 100 || dial < 0 {
            dial = if dial >= 100 { dial - 100 } else { dial + 100 };
            zeroes += 1;
        }
    });
    zeroes
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/01.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/01");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
