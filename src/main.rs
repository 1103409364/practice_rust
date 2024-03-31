use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg stands for debug print。dbg 是 Rust 标准库中的一个宏，可以打印变量的值
    // env::args 读取到的参数中第一个就是程序的可执行路径名。
    // dbg!(args);
    // 对 build 返回的 `Result` 进行处理
    // let config = Config::from(&args)?; //  这种方式，需要标出 main 的返回值 -> Result<(), Box<dyn Error>>。错误向上传播，没有处理
    let config = Config::from(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
    // 从代码惯例的角度出发，new 往往不会失败，毕竟新建一个实例没道理失败，因此修改为 from 会更加合适
    fn from(args: &[String]) -> Result<Self, &'static str> {
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
