pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let str_num = num.to_string();
    let digits = str_num.len() as u32;

    str_num.chars().fold(0, |acct, cur| {
        // acct + cur.to_string().parse::<u32>().unwrap_or(0).pow(digits)
        acct + cur.to_digit(10).unwrap_or(0).pow(digits)
    }) == num
}
