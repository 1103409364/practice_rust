// 所有权
// 对于实现了 Copy 特征的类型，例如 i32，那类型的值会被拷贝到 HashMap 中。而对于有所有权的类型，例如 String，它们的值的所有权将被转移到 HashMap 中。

// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
use std::collections::HashMap;
fn main() {
  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello".to_string();
  let mut m2 = HashMap::new();
  // 所有权在这里发生了转移
  m2.insert(v2.clone(), v1);

  assert_eq!(v2, "hello");

   println!("Success!")
}