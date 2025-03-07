pub fn reply(mut message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    message = message.trim();
    // 问号 大写 空，三种情况
    match (
        message.ends_with("?"),
        message.to_uppercase() == message && !message.chars().all(|x| !x.is_alphabetic()),
        message == "",
    ) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (false, false, false) => "Whatever.",
    }
}
