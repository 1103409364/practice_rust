pub fn abbreviate(phrase: &str) -> String {
    phrase
        // .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .replace(['-', '_'], " ")
        .split_whitespace()
        // flat_map 接受一个闭包，该闭包将迭代器中的每个元素转换为一个新的迭代器，然后 flat_map 将这些新的迭代器连接成一个扁平的迭代器。
        .flat_map(|mut word| {
            // word.chars().skip_while(|c| c.is_uppercase()) skip_while 接受一个闭包作为参数。这个闭包会被应用于迭代器中的每个元素，直到闭包返回 false 为止。跳过所有从起始位置开始的连续大写字母字符。
            if word == word.to_uppercase() {
                word = &word[0..1];
            }

            word.chars()
                .take(1) // take 返回 Take 是迭代器（实现了 Iterator trait）
                .chain(word.chars().skip(1).filter(|c| c.is_uppercase()))
        })
        .map(|c| c.to_uppercase().to_string())
        .collect()
}
