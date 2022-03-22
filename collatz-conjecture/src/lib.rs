pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n => {
            let mut step = 0_u64;
            let mut n = n;
            while n > 1 {
                if (n & 1) == 1 {
                    let m = n.checked_mul(3);
                    if m.is_none() || m?.checked_add(1).is_none() {
                        return None;
                    }
                    n = m.unwrap() + 1;
                } else {
                    n /= 2;
                }
                step += 1;
            }
            Some(step)
        }
    }

}
