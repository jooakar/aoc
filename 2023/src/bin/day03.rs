mod utils;
use utils::find_num;

fn part1(input: String) -> String {
    let lines: usize = input.lines().count();
    let length: usize = input.lines().next().unwrap().len();
    let mut res = 0;
    let options = [0, 1, 2];
    let dirs = options.iter().flat_map(|y| options.map(|z| (y, z)));

    let mut add = vec![vec![false; length]; lines];
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                dirs.clone().for_each(|(x,y)| add[row + x - 1][col + y - 1] = true);
            }
        }
    }

    for (row, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut possible = false;
        for (col, char) in line.chars().enumerate() {
            match char.to_digit(10) {
                None => {
                    if possible { res += num; }
                    num = 0;
                    possible = false;
                },
                Some(digit) => {
                    possible = possible || add[row][col];
                    num *= 10;
                    num += digit;
                },
            }
        }
        if possible { res += num; }
    }

    res.to_string()
}

fn part2(input: String) -> String {
    let lines: usize = input.lines().count();
    let length: usize = input.lines().next().unwrap().len();
    let mut res = 0;
    let options = [0, 1, 2];
    let dirs = options.iter().flat_map(|y| options.map(|z| (y, z)));
    let chars = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for row in 0..lines {
        for col in 0..length {
            if chars[row][col] != '*' {
                continue;
            }
            let mut nums = dirs
                .clone()
                .filter_map(|(x,y)| find_num(chars[row + x - 1].clone(), col + y - 1))
                .collect::<Vec<u32>>();
            nums.dedup();
            if nums.len() == 2 {
                res += nums.iter().product::<u32>();
            }
        }
    }

    res.to_string()
}

fn main() {
    let input: String = include_str!("../../input/03").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
