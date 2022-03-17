pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    let mut res = String::new();
    if n % 3 == 0 {
        res += "Pling";
    }
    if n % 5 == 0 {
        res += "Plang";
    }
    if n % 7 == 0 {
        res += "Plong";
    }
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        res += (n).to_string().as_str();
    }
    res
}
