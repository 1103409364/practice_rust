// 孤儿原则
// 关于孤儿原则的详细介绍请参见特征定义与实现的位置孤儿规则 和 在外部类型上实现外部特征。

use std::fmt;

// 定义一个 newtype `Pretty`
struct Pretty(String);
impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
