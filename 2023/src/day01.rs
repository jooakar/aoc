fn part1(input: String) -> String {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| vec.first().unwrap() * 10 + vec.last().unwrap())
        .sum::<u32>()
        .to_string()
}

fn part2(input: String) -> String {
    let modified = input.lines()
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .collect::<Vec<String>>()
        .join("\n");
    part1(modified)
}

fn main() {
    let input: String = include_str!("../input/01").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input.clone()));
}
