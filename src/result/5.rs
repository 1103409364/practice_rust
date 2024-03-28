use std::num::ParseIntError;

// 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
// 但是这种写法实在过于啰嗦..
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => match n2_str.parse::<i32>() {
            Ok(n2) => Ok(n1 * n2),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

// 重写上面的 `multiply` ，让它尽量简洁
// 提示：使用 `and_then` 和 `map`
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    // let n1 = n1_str.parse::<i32>()?;
    // let n2 = n2_str.parse::<i32>()?;
    // 使用 and_then 对 Result 进行处理
    n1_str
        .parse::<i32>()
        .and_then(|x| n2_str.parse::<i32>().and_then(|y| Ok(x * y)))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply1("10", "2");
    print(twenty);

    // 下面的调用会提供更有帮助的错误信息
    let tt = multiply("t", "2");
    print(tt);

    println!("Success!")
}
