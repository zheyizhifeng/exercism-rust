pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    let mut sum: u32 = (1..=limit).sum();
    for i in 1..=limit {
        if is_not_multiple(i, factors) {
            sum -= i;
        }
    }
    sum
}

fn is_not_multiple(n: u32, factors: &[u32]) -> bool{
    for x in factors {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}