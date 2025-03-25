#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (0, _) => return Err(Error::InvalidInputBase),
        (_, 0) => return Err(Error::InvalidOutputBase),
        (1, _) => return Err(Error::InvalidInputBase),
        (_, 1) => return Err(Error::InvalidOutputBase),
        _ => {}
    }

    if number.iter().any(|&n| n >= from_base) {
        return Err(Error::InvalidDigit(*number.iter().max().unwrap()));
    }

    let mut n = number.iter().fold(0, |acc, n| acc * from_base + n);

    if n == 0 {
        return Ok(vec![0]);
    }

    let mut ret = vec![];
    while n > 0 {
        ret.push(n % to_base);
        n /= to_base;
    }

    Ok(ret.into_iter().rev().collect())
}
