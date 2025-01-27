/* 使用三种方法修复下面的错误  */
// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

// fn invalid_output<'a>() -> String {
//     String::from("foo")
// }
fn invalid_output<'a>(s: &'a String) -> &'a String {
    s
}

fn main() {
}