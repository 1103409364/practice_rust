// 三个 &，报错 the size for values of type `str` cannot be known at compilation time
// 编译器无法确定 str 的大小，无法直接使用，需要使用引用 &str
fn f2(&&x: &&&str) {
    println!("{:?}", x);
}

fn main() {
    let s = "Hello, world!";
    // 如果下面有调用 f1，可以省略类型标注 : &&str，可以自动推断。否则需要显式声明
    // f2 普通函数不能省略类型标注
    let f1 = |&x: &&str| println!("{:?} ", x);
    f1(&s);
    f2(&&s);
}
