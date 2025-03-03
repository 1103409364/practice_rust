pub fn factors(mut n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")
    let mut factors = Vec::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}
