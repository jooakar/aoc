pub fn nums<T: std::str::FromStr>(str: &str) -> Vec<T> {
    str.split(|c: char| !c.is_ascii_digit())
        .flat_map(|s| s.parse())
        .collect()
}
