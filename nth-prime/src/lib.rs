pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    (1..)
        .filter(|&x| is_prime(x))
        .take((n + 1) as usize)
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
}
fn is_prime(x: u32) -> bool {
    if x < 2 {
        false
    } else if x == 2 {
        true
    } else {
        for i in 2..=x / 2 {
            if x % i == 0 {
                return false;
            }
        }
        true
    }
}
