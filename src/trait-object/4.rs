// 静态分发和动态分发Static and Dynamic dispatch
// 关于这块内容的解析介绍，请参见 Rust语言圣经。

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// 通过泛型实现以下函数
fn static_dispatch<T: Foo>(x: T) -> String {
    x.method()
}

// 通过特征对象实现以下函数
fn dynamic_dispatch(p: &dyn Foo) -> String {
    p.method()
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!")
}
