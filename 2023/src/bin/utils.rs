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
pub fn nums(str: &str) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    let mut curr = 0;
    for c in str.chars() {
        match c.to_digit(10) {
            Some(n) => { curr *= 10; curr += n; }
            None => { if curr > 0 { res.push(curr); }; curr = 0; }
        }
    }
    if curr > 0 { res.push(curr); }
    res
}

