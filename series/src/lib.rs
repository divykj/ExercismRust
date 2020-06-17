pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        Vec::with_capacity(0)
    } else {
        (0..digits.len() - len + 1)
            .map(|i| digits[i..(i + len)].to_string())
            .collect()
    }
}
