use std::fmt::Display;

fn part1(input: String) -> impl Display {
    input.trim()
        .split(',')
        .map(|s| s.bytes().fold(0, |acc, b| ((acc + b as usize) * 17) % 256))
        .sum::<usize>()
}

fn part2(input: String) -> impl Display {
    let mut boxes: Vec<Vec<_>> = vec![vec![]; 256];
    input.trim()
        .split(',')
        .map(|s| s.split_once(&['-','='][..]).unwrap())
        .for_each(|(id, op)| {
            let hash = id.bytes().fold(0, |acc, b| ((acc + b as usize) * 17) % 256);
            let index = boxes[hash].iter().position(|x: &(&str, usize)| x.0 == id);
            match op.parse::<usize>() {
                Ok(val) => {
                    match index {
                        Some(i) => boxes[hash][i] = (id, val),
                        None => boxes[hash].push((id, val)),
                    }
                }
                Err(_) => {
                    if let Some(i) = index { boxes[hash].remove(i); }
                }
            };
        });
    boxes.iter()
        .enumerate()
        .map(|(b, contents)| {
            contents.iter()
                .enumerate()
                .map(|(slot, (_, focal))| (b+1) * (slot+1) * focal)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: String = include_str!("../../input/15.ex").to_string();
        println!("EXAMPLE 1: {}", part1(example.clone()));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: String = include_str!("../../input/15").to_string();
        println!("PART 1: {}", part1(input.clone()));
        println!("PART 2: {}", part2(input));
    }
}
