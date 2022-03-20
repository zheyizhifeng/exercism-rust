// fn is_prime(x: u64) -> (bool, u64) {
//     if x <= 1 {
//         (false, 1)
//     } else {
//         let median_index = (x as f64).sqrt() as u64;

//         for i in 2..=median_index {
//             if x % i == 0 {
//                 return (false, i);
//             }
//         }
//         (true, x)
//     }
// }
pub fn factors(n: u64) -> Vec<u64> {
    // if n <= 1 {
    //     return Vec::new();
    // } else {
    //     let (n_is_prime, factor) = is_prime(n);
    //     if n_is_prime {
    //         return vec![factor];
    //     } else {
    //         return [vec![factor], factors(n/factor)].concat();
    //     }
    // }

    let mut n = n;
    let mut result = Vec::new();
    // let median_index = (n as f64).sqrt() as u64;
    let mut i = 2;
    while n > 1 && i <= n {
        if n % i == 0 {
            result.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    result
}
