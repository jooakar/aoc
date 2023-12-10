use itertools::Itertools;

fn card_value(c: char, p2: bool) -> usize {
    if c.is_ascii_digit() { return c.to_digit(10).unwrap() as usize; }
    match c {
        'T' => 10,
        'J' => if p2 { 1 } else { 11 },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn hand_value(mut counts: Vec<usize>, jokers: usize) -> usize {
    if jokers == 5 { return 6; }
    counts[0] += jokers;
    match counts[..] {
        [5] => 6,
        [4, ..] => 5,
        [3, 2] => 4,
        [3, ..] => 3,
        [2, 2, ..] => 2,
        [2, ..] => 1,
        _ => 0,
    }
}

fn part1(input: String) -> String {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|t| (t.0, t.1.parse::<usize>().unwrap()))
        .map(|(hand, bet)| {
            let counts: Vec<usize> = hand
                .chars()
                .counts()
                .into_values()
                .sorted_by_key(|x| std::cmp::Reverse(*x))
                .collect();
            let tiebreaker: usize = hand
                .chars()
                .fold(0, |acc, c| acc * 100 + card_value(c, false));
            (hand_value(counts, 0), tiebreaker, bet)
        })
        .collect();
    hands.sort_by_key(|h| h.1);
    hands.sort_by_key(|h| h.0);
    hands.iter().enumerate().map(|(i, (_, _, bet))| bet * (i + 1)).sum::<usize>().to_string()
}

fn part2(input: String) -> String {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|t| (t.0, t.1.parse::<usize>().unwrap()))
        .map(|(hand, bet)| {
            let jokers = hand.chars().filter(|&c| c == 'J').count();
            let counts: Vec<usize> = hand
                .chars()
                .filter(|&c| c != 'J')
                .counts()
                .into_values()
                .sorted_by_key(|x| std::cmp::Reverse(*x))
                .collect();
            let tiebreaker: usize = hand
                .chars()
                .fold(0, |acc, c| acc * 100 + card_value(c, true));
            (hand_value(counts, jokers), tiebreaker, bet)
        })
        .collect();
    hands.sort_by_key(|h| h.1);
    hands.sort_by_key(|h| h.0);
    hands.iter().enumerate().map(|(i, (_, _, bet))| bet * (i + 1)).sum::<usize>().to_string()
}

fn main() {
    let input: String = include_str!("../../input/07").to_string();
    println!("PART 1: {}", part1(input.clone()));
    println!("PART 2: {}", part2(input));
}
