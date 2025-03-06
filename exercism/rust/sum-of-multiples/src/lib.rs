pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    factors
        .iter()
        .fold(vec![], |mut acc, factor| {
            let mut i = 1;
            let mut x = *factor * i;
            while x < limit && *factor != 0 {
                if !acc.contains(&x) {
                    acc.push(x)
                }
                i += 1;
                x = *factor * i;
            }
            acc
        })
        .into_iter()
        .sum()
}
