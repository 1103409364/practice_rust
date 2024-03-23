// impl Trait 语法非常直观简洁，但它实际上是特征约束的语法糖。

// 当使用泛型参数时，我们往往需要为该参数指定特定的行为，这种指定方式就是通过特征约束来实现的。

fn main() {
    assert_eq!(sum(1, 2), 3);
}

// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
