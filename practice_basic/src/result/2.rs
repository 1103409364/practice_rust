// ? 跟 unwrap 非常像，但是 ? 会返回一个错误，而不是直接 panic

use std::num::ParseIntError;

// 使用 `?` 来实现 multiply
// 不要使用 unwrap !
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    // let n1 = n1_str.parse::<i32>();
    // let n2 = n2_str.parse::<i32>();
    // Ok(n1.unwrap() * n2.unwrap())
    // match n1 {
    //     Ok(x) => match n2 {
    //         Ok(y) => Ok(x * y),
    //         Err(e) => return Err(e),
    //     },
    //     Err(e) => return Err(e),
    // }
    // parse 的结果是 Result<T, E>。如果结果是 Ok(T)，则把 T 赋值给 n1、n2，如果结果是 Err(E)，则返回该错误，所以 ? 特别适合用来传播错误。
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!")
}
