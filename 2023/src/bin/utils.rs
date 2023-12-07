// Finds the num that exists in the given index
pub fn find_num(chars: Vec<char>, mut col: usize) -> Option<u32> {
    if !chars[col].is_ascii_digit() {
        return None;
    }
    let n = chars.len();
    while col > 0 {
        if chars[col - 1].is_ascii_digit() { col -= 1; } 
        else { break; }
    }
    let mut res = 0;
    while col < n && chars[col].is_ascii_digit() {
        res *= 10;
        res += chars[col].to_digit(10).unwrap();
        col += 1;
    }
    Some(res)
}


// Finds all (positive) numbers in the string
pub fn nums<T: std::str::FromStr>(str: &str) -> Vec<T> {
    str.split_ascii_whitespace()
        .flat_map(|s| s.parse())
        .collect()
}

