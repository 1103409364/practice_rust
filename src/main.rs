use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg stands for debug print。dbg 是 Rust 标准库中的一个宏，可以打印变量的值
    // env::args 读取到的参数中第一个就是程序的可执行路径名。
    // dbg!(args);
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("with text:\n{contents}");
}
// Config 中存储的并不是 &str 这样的引用类型，而是一个 String 字符串，也就是 Config 并没有去借用外部的字符串，而是拥有内部字符串的所有权。
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Self { query, file_path }
    }
}
