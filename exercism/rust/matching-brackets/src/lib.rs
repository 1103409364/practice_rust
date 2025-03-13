pub fn brackets_are_balanced(string: &str) -> bool {
    // todo!("Check if the string \"{string}\" contains balanced brackets");
    const LEFT_BRACKETS: [char; 4] = ['(', '[', '{', '<'];
    const RIGHT_BRACKETS: [char; 4] = [')', ']', '}', '>'];
    let mut stack = vec![];
    for c in string.chars() {
        if LEFT_BRACKETS.contains(&c) {
            stack.push(c);
        }
        if RIGHT_BRACKETS.contains(&c) {
            if let Some(left) = stack.pop() {
                if LEFT_BRACKETS.iter().position(|&x| x == left)
                    != RIGHT_BRACKETS.iter().position(|&x| x == c)
                {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
