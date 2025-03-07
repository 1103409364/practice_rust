pub fn reply(mut message: &str) -> &str {
    message = message.trim();
    // 三种情况，分类讨论：问号、大写、空
    match (
        message.ends_with("?"),
        message.to_uppercase() == message && !message.chars().all(|x| !x.is_alphabetic()),
        message.is_empty(),
    ) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (false, false, false) => "Whatever.",
    }
}
