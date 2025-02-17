// 为自定义类型实现 From 特征
// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // 实现 `from` 方法
    fn from(value: i32) -> Self {
        Self { value }
    }
}

// 填空
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);
    // .into() 类型转换 实现了 From trait，会自动实现 Into trait
    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!")
}
