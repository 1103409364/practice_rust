// 类型别名
// 如果我们要在代码中到处使用 std::result::Result<T, ParseIntError> ，那毫无疑问，代码将变得特别冗长和啰嗦，对于这种情况，可以使用类型别名来解决。

// 例如在标准库中，就在大量使用这种方式来简化代码: io::Result.

use std::num::ParseIntError;

// 填空
type Res<T> = Result<T, ParseIntError>;

// 使用上面的别名来引用原来的 `Result` 类型
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 同样, 这里也使用了类型别名来简化代码
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!")
}