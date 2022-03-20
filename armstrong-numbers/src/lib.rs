pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut sum = 0;
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    let mut n = num;
    while n > 0 {
        sum += (n % 10).pow(count);
        n /= 10;
    }
    sum == num
}
