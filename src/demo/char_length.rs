fn main() {
    // String 的 len() 方法返回的是字符串中【字节】的长度，
    // 而 chars().count() 方法返回的是字符串中 Unicode 字符的个数。
    let message = "1二三四五六七八九十";
    let byte_length = message.len();
    let char_count = message.chars().count();
    println!("Byte length: {}, Char count: {}", byte_length, char_count);
}