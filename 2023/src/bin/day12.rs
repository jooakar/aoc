fn check(which: &str, groups: &[usize]) -> bool {
    which.split(&['.', '?'][..])
        .filter(|&s| !s.is_empty())
        .map(|g| g.len())
        .collect::<Vec<usize>>()
        .eq(groups)
}

fn traverse(which: Vec<char>, groups: &[usize], i: usize, gi: usize) -> usize {
    if i >= which.len() || gi >= groups.len() {
        return if check(&which.into_iter().collect::<String>(), groups) { 1 } else { 0 }
    }
    match which[i] {
        '.' => traverse(which, groups, i+1, gi),
        '#' => {
            let end = i + groups[gi];
            if end > which.len()
                || !which[i..end].iter().all(|&c| c == '#' || c == '?') { 0 }
            else if gi + 1 == groups.len() && end == which.len() { 1 }
            else if end < which.len() && which[end] == '#' { 0 }
            else { 
                let mut next = which.clone();
                next[i..end].fill('#');
                traverse(next, groups, end + 1, gi + 1)
            }
        },
        '?' => {
            let mut hash = which.clone();
            let mut dot = which.clone();
            hash[i] = '#';
            dot[i] = '.';
            traverse(dot, groups, i, gi) + traverse(hash, groups, i, gi)
        },
        _ => unreachable!()
    }
}

fn part1(input: String) -> String {
    let mut res = 0;
    for line in input.lines() {
        let (row, groups) = line.split_once(' ').unwrap();
        let groups = groups.split(',').flat_map(|s| s.parse()).collect::<Vec<_>>();
        res += traverse(row.chars().collect(), &groups, 0, 0);
    }
    res.to_string()
}


fn part2(input: String) -> String {
    let mut res = 0;
    for line in input.lines() {
        let (row, groups) = line.split_once(' ').unwrap();
        let groups = groups.split(',').flat_map(|s| s.parse()).collect::<Vec<_>>().repeat(5);
        let row = [row, row, row, row, row].join("?");
        let result = traverse(row.chars().collect(), &groups, 0, 0);
        println!("{line} - {result}");
        res += result;
    }
    res.to_string()
}

fn main() {
    let example: String = include_str!("../../input/12.ex").to_string();
    let input: String = include_str!("../../input/12").to_string();
    println!("EXAMPLE 1: {}", part1(example.clone()));
    println!("PART 1: {}", part1(input.clone()));
    println!("EXAMPLE 2: {}", part2(example));
    println!("PART 2: {}", part2(input));
}
