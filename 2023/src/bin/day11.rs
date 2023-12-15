fn solution(input: String, empty: usize) -> String {
    let (h, w) = (input.lines().count(), input.lines().nth(1).unwrap().len());
    let mut empty_cols = vec![true; w];
    let mut empty_rows = vec![true; h];
    // Find empty rows & cols
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                empty_rows[row] = false;
                empty_cols[col] = false;
            }
        }
    }
    // Find star locations
    let (mut rowoffset, mut coloffset) = (0, 0);
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for (row, line) in input.lines().enumerate() {
        if empty_rows[row] { rowoffset += empty; }
        coloffset = 0;
        for (col, c) in line.chars().enumerate() {
            if empty_cols[col] { coloffset += empty; }
            if c == '#' {
                galaxies.push((row + rowoffset, col + coloffset));
            }
        }
    }
    // Find distances
    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let rowdiff = galaxies[i].0.abs_diff(galaxies[j].0);
            let coldiff = galaxies[i].1.abs_diff(galaxies[j].1);
            res += rowdiff + coldiff;
        }
    }

    res.to_string()
}

fn main() {
    let input: String = include_str!("../../input/11").to_string();
    println!("PART 1: {}", solution(input.clone(), 1));
    println!("PART 2: {}", solution(input, 1000000 - 1));
}
