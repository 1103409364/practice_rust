use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg stands for debug print。dbg 是 Rust 标准库中的一个宏，可以打印变量的值
    // env::args 读取到的参数中第一个就是程序的可执行路径名。
    // dbg!(args);
    // 对 from 返回的 `Result` 进行处理
    // let config = Config::from(&args)?; //  这种方式，需要标出 main 的返回值 -> Result<(), Box<dyn Error>>。错误向上传播，没有处理
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::from(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // process::exit(1) 来终结进程，其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    // if let 模式匹配 Err(e) 匹配到错误
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
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
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text:\n{contents}");
    Ok(())
}
