use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);
    let mut result = HashSet::new();
    // for i in 1..sum / 3 {
    //     for j in i + 1..sum / 2 {
    //         let diff = sum - i - j;
    //         if diff > j && i * i + j * j == diff * diff {
    //             result.insert([i, j, diff]);
    //         }
    //     }
    // }
    /*
     * (where n is a one-character alias for sum)
     * c = n - a - b
     * a^2 + b^2 = c^2
     * a^2 + b^2 = n^2 - 2an - 2bn + a^2 + 2ab + b^2
     * 2bn - 2ab = n^2 - 2an
     * 2b(n - a) = n(n-2a)
     * b = n(n-2a) / 2(n-a)
     * b = (n(n-a) - an) / 2(n-a)
     */
    // 假设 a < b < c
    for a in 1..sum / 3 {
        let b: f64 = sum as f64 / 2.0 - (a as f64 * sum as f64) / (2.0 * (sum - a) as f64);
        if b.floor() as f64 == b {
            let b = b as u32;
            if a >= b {
                continue;
            }
            let c = sum - a - b;
            result.insert([a, b, c]);
        }
    }
    result
}
