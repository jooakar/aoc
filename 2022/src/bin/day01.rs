use std::cmp::max;

fn part1(input: String) -> String {
    let mut res: u32 = 0;
    for block in input.split("\n\n") {
        let calories = block
            .lines()
            .map(|line| line.parse::<u32>().expect("parse error"))
            .sum();
        res = max(calories, res);
    }

    res.to_string()
}

fn part2(input: String) -> String {
    let mut res: [u32; 3] = [0; 3];
    for block in input.split("\n\n") {
        let calories = block
            .lines()
            .map(|line| line.parse::<u32>().expect("parse error"))
            .sum();
        let smallest = res.iter().enumerate().map(|(x,y)| (y,x)).min().unwrap().1;
        if res[smallest] < calories { res[smallest] = calories }
    }

    res.into_iter().sum::<u32>().to_string()
}

fn main() {
    let input: String = include_str!("../../input/01").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
