fn part1(input: String) -> String {
    let mut ans = 0;

    for (i, line) in input.lines().enumerate() {
        let (_, data) = line.split_once(": ").unwrap();
        let games = data.split("; ").collect::<Vec<&str>>();
        let mut possible = true;
        for game in games {
            for color in game.split(", ") {
                let count: u32 = color.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
                let max = match color.split(' ').nth(1).unwrap() {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => unreachable!()
                };
                possible = possible && count <= max;
            }
        }
        if possible { ans += i + 1 };
    }

    ans.to_string()

}

use std::cmp::max;

fn part2(input: String) -> String {
    let mut ans = 0;

    for line in input.lines() {
        let (mut red, mut green, mut blue) = (0,0,0);
        let (_, data) = line.split_once(": ").unwrap();
        let games = data.split("; ").collect::<Vec<&str>>();
        for game in games {
            for color in game.split(", ") {
                let count: u32 = color.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
                match color.split(' ').nth(1).unwrap() {
                    "red" => red = max(red, count),
                    "green" => green = max(green, count),
                    "blue" => blue = max(blue, count),
                    _ => unreachable!()
                };
            }
        }
        ans += red * green * blue;
    }

    ans.to_string()

}

fn main() {
    let input: String = include_str!("../../input/02").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
