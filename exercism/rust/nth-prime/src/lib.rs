pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let mut count = 0;
    let mut num = 2;
    while count < n {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}

fn is_prime(n: u32) -> bool {
    match n {
        ..2 => false,
        2 => true,
        3 => true,
        _ => {
            let mut i = 2;
            while i <= n / 2 {
                if n % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        }
    }
}
