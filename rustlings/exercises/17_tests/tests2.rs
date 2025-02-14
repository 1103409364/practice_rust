// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        let base: u64 = 2;
        assert_eq!(base.pow(2), power_of_2(2));
        assert_eq!(16, power_of_2(4));
        assert_eq!(2048, power_of_2(11));
        assert_eq!(8_589_934_592, power_of_2(33));
    }
}
