fn part1(input: String) -> String {
    let mut res: u32 = 0;
    let values = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (b'A'.abs_diff(a.chars().next().unwrap() as u8) + 1, b'X'.abs_diff(b.chars().next().unwrap() as u8) + 1));

    for value in values {
        res += u32::from(value.1);
        if value.0 == value.1  { res += 3; }
        else if (value.1 + 1) % 3 == value.0 - 1 { res += 6; }
        else { res += 0; }
    }
    
    res.to_string()
}

fn part2(input: String) -> String {
    let mut res: u32 = 0;
    let values = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (b'A'.abs_diff(a.chars().next().unwrap() as u8) + 1, b'X'.abs_diff(b.chars().next().unwrap() as u8) + 1));

    for value in values {
        if value.1 == 1  { res += u32::from((value.0 + 1) % 3 + 1); }
        else if value.1 == 2  {
            res += u32::from(value.0);
            res += 3;
        } else {
            res += u32::from(value.0 % 3 + 1);
            res += 6;
        }
    }
    
    res.to_string()
}

fn main() {
    let input: String = include_str!("../../input/02").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
