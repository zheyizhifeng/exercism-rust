pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        vec![]
    } else {
        let mut res = Vec::new();
        for i in 0..=digits.len() - len {
            res.push((&digits[i..i + len]).to_string());
        }
        res
    }
}
