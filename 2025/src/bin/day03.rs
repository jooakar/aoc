use std::fmt::Display;

fn part1(input: &str) -> impl Display {
    solution(input, 2)
}

fn part2(input: &str) -> impl Display {
    solution(input, 12)
}

fn solution(input: &str, chars: usize) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut score = 0;
            let mut left = 0;
            for i in (0..chars).rev() {
                score *= 10;
                let (i, b) = l[0..(l.len() - i)]
                    .chars()
                    .enumerate()
                    .skip(left)
                    .reduce(|acc, curr| if curr.1 > acc.1 { curr } else { acc })
                    .unwrap();
                left = i + 1;
                score += b.to_digit(10).unwrap() as u64;
            }
            score
        })
        .sum::<u64>()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/03.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/03");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
