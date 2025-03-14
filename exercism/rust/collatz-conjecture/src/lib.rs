pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    match n {
        0 => None,
        mut n => {
            while n != 1 {
                count += 1;
                match n % 2 {
                    0 => n /= 2,
                    _ => n = n * 3 + 1,
                }
                if count > 1000 {
                    return None;
                }
            }
            Some(count)
        }
    }
}
