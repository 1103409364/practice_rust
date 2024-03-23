// 修复代码中的错误
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
// Unit 需要实现 Debug、PartialOrd、PartialEq 才能使用 cmp_display 方法。cmp_display 的实现，传递范型参数 T 给 Pair，此时的 T 受到特质约束，因此可以调用 Pair 的 cmp_display 方法。
#[derive(Debug, PartialOrd, PartialEq)]
struct Unit(i32);

fn main() {
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    pair.cmp_display();
}
