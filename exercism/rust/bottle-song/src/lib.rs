use std::collections::HashMap;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    let number_map: HashMap<u32, String> = HashMap::from([
        (0, "no".to_string()),
        (1, "one".to_string()),
        (2, "two".to_string()),
        (3, "three".to_string()),
        (4, "four".to_string()),
        (5, "five".to_string()),
        (6, "six".to_string()),
        (7, "seven".to_string()),
        (8, "eight".to_string()),
        (9, "nine".to_string()),
        (10, "ten".to_string()),
    ]);

    let s = (1..=take_down)
        .map(|n| {
            let s1 = format!(
                "{} green bottles hanging on the wall,\n",
                // 首字母大写
                number_map
                    .get(&(start_bottles - n + 1))
                    .unwrap_or(&"".to_string())
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        match i {
                            0 => c.to_uppercase().to_string(),
                            _ => c.to_string(),
                        }
                    })
                    .collect::<String>()
            );
            let s2 = format!(
                "There'll be {} green bottles hanging on the wall.",
                number_map
                    .get(&(start_bottles - n))
                    .unwrap_or(&"".to_string())
            );
            s1.repeat(2) + "And if one green bottle should accidentally fall,\n" + &s2
        })
        .collect::<String>();
    s
}
