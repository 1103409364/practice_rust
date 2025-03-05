pub fn raindrops(n: u32) -> String {
    // todo!("what sound does Raindrop #{n} make?")
    let s = [3, 5, 7]
        .map(|x| match x {
            3 => match n % 3 {
                0 => "Pling".to_string(),
                _ => "".to_string(),
            },
            5 => match n % 5 {
                0 => "Plang".to_string(),
                _ => "".to_string(),
            },
            7 => match n % 7 {
                0 => "Plong".to_string(),
                _ => "".to_string(),
            },
            _ => "".to_string(),
        })
        .join("");

    if s.is_empty() { n.to_string() } else { s }
}
