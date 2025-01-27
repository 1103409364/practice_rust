// TryFrom/TryInto
// 类似于 From 和 Into, TryFrom 和 TryInto 也是用于类型转换的泛型特征。

// 但是又与 From/Into 不同, TryFrom 和 TryInto 可以对转换后的失败进行处理，然后返回一个 Result。

// TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
// use std::convert::TryInto;

fn main() {
    let n: i16 = 256;

    // Into 特征拥有一个方法`into`,
    // 因此 TryInto 有一个方法是 ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!(
                "there is an error when converting: {:?}, but we catch it",
                e.to_string()
            );
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!")
}
