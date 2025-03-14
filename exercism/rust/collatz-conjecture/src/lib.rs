pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    match n {
        0 => None,
        mut n => {
            while n != 1 {
                count += 1;
                n = match n % 2 {
                    0 => n / 2,
                    _ => n.checked_mul(3)?.checked_add(1)?, // Add overflow protection
                };
                if count > 1000 {
                    return None;
                }
            }
            Some(count)
        }
    }
}
