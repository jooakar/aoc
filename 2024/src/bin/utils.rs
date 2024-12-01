pub fn nums<T: std::str::FromStr>(str: &str) -> Vec<T> {
    str.split_ascii_whitespace()
        .flat_map(|s| s.parse())
        .collect()
}
