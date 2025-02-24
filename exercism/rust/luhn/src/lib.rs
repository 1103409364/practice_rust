/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    let code_vec: Vec<i32> = code
        .split("")
        .filter(|&s| s != " " && s != "")
        .map(|s| s.parse::<i32>().unwrap_or(-1))
        .collect();

    if code_vec.len() <= 1 || code_vec.iter().any(|n| *n == -1) {
        return false;
    }

    let db_sum = code_vec
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acct, (i, n)| match i % 2 {
            0 => acct + n,
            _ => acct + if n * 2 > 9 { n * 2 - 9 } else { n * 2 },
        });

    db_sum % 10 == 0
}
