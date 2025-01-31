// 🌟🌟 当将任何数值转换成无符号整型 T 时，如果当前的数值不在新类型的范围内，
// 我们可以对当前数值进行加值或减值操作( 增加或减少 T::MAX + 1 )，直到最新的值在新类型的范围内，
// 假设我们要将 300 转成 u8 类型，由于u8 最大值是 255，
// 因此 300 不在新类型的范围内并且大于新类型的最大值，因此我们需要减去 T::MAX + 1，也就是 300 - 256 = 44。
#[allow(overflowing_literals)]

fn main() {
    assert_eq!(1000 as u16, 1000); // u16::MAX is 65535 and 1000 is within range

    assert_eq!(1000 as u8, 232); // u8::MAX is 255 and 1000 is greater than u8::MAX. We need to subtract T::MAX + 1, which is 1000 - 256 = 744. 744 - 256 = 488 488 - 256 = 232

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255); // u8::MAX is 255 and -1 is less than 0. We need to add T::MAX + 1, -1 + 256 = 255.

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
