use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg stands for debug print dbg 是 Rust 标准库中的一个宏，可以打印变量的值
    dbg!(args);
}
