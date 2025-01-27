// 生命周期消除( Elision )
// 有一些生命周期的标注方式很常见，因此编译器提供了一些规则，可以让我们在一些场景下无需去标注生命周期，既节省了敲击键盘的繁琐，又能提升可读性。

// 这种规则被称为生命周期消除规则( Elision )，该规则之所以存在，仅仅是因为这些场景太通用了，为了方便用户而已。事实上对于借用检查器而言，该有的生命周期一个都不能少，只不过对于用户而言，可以省去一些。
/* 移除所有可以消除的生命周期标注 */

fn nput(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: & i32) -> & i32 {
    x
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}
