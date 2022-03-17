pub fn square(s: u32) -> u64 {
    if s <= 0 || s >= 65 {
        panic!("ISquare must be between 1 and 64");
    }
    // unimplemented!("grains of rice on square {}", s);
    // 2 ^ (s-1)
    (1_u64) << (s - 1)
}

pub fn total() -> u64 {
    // unimplemented!();
    (1..=64).map(|x| square(x)).sum::<u64>()
}
