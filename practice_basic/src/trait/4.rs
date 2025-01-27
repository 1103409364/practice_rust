// 修复错误，不要修改 `main` 中的代码!
use std::ops;

struct Foo;
struct Bar;
#[derive(Debug, PartialEq)]
struct FooBar;

#[derive(Debug, PartialEq)]
struct BarFoo;

// 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    // 关联类型 Output 定义了 add 函数的返回类型
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}
// 下面的代码实现了自定义类型的相减： Foo - Bar = BarFoo
impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}
// 下面的代码实现了自定义类型的相减： Bar - Foo = BarFoo
// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }

fn main() {
    // 不要修改下面代码
    // 你需要为 FooBar 派生一些特征来让代码工作
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!")
}
