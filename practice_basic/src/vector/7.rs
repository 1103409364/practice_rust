// 在 Vec 中存储不同类型的元素
// Vec 中的元素必须是相同的类型，例如以下代码会发生错误:
// fn main() {
//    let v = vec![1, 2.0, 3];
// }

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // 填空
    let v : Vec<IpAddr>= vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];

    // 枚举的比较需要派生 PartialEq 特征
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}