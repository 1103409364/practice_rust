use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    // 使用 HashMap 提高查找效率
    let brackets_map: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();

    let mut stack = Vec::new();

    for c in string.chars() {
        if brackets_map.contains_key(&c) {
            // 如果是左括号，压入栈中
            stack.push(c);
        } else if brackets_map.values().any(|&right| right == c) {
            // 如果是右括号，检查是否匹配
            match stack.pop() {
                Some(left) => {
                    if brackets_map.get(&left) != Some(&c) {
                        return false;
                    }
                }
                None => return false, // 栈为空，没有匹配的左括号
            }
        }
        // 忽略非括号字符
    }

    stack.is_empty() // 确保所有左括号都被匹配
}
