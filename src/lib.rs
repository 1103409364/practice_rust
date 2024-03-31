use std::error::Error;
use std::fs;

// Config 中存储的并不是 &str 这样的引用类型，而是一个 String 字符串，也就是 Config 并没有去借用外部的字符串，而是拥有内部字符串的所有权。
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // 从代码惯例的角度出发，new 往往不会失败，毕竟新建一个实例没道理失败，因此修改为 from 会更加合适
    pub fn from(args: &[String]) -> Result<Self, &'static str> {
        // 缺少参数报错，防止数组越界
        if args.len() < 3 {
            // panic!("not enough arguments")
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text:\n{contents}");
    Ok(())
}
