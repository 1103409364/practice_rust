fn main() {
    let arr: [u64; 13] = [0; 13]; // 初始化数组使用 13 个 0
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    // 创建了一个指向数组 arr 的地址的常量原始指针 a，允许我们以底层方式处理数组第一个元素的内存地址。
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    println!("{:?}", b);
    // rust针对数组的类型转换应用到了数组的每一个元素，即从u64变成u8 输出结果是数组占据的空间的大小
    // u64 一个元素占8个字节，所以数组的大小为 13 * 1 * 8 = 104 字节。
    // u8 一个元素占1个字节，所以数组的大小为 13 * 1 = 13 字节。
    unsafe { assert_eq!(std::mem::size_of_val(&*b), 13) }
}
// 使用13个0初始化了一个名为"arr"的数组，数组的每个元素都是无符号64位整数。
// 使用assert_eq!宏断言"arr"数组的大小是否等于8（u64的大小）乘以13，确保数组的大小是预期的。
// 取"arr"数组的地址，并将其赋值给了类型为*const [u64]的指针"a"。
// 将指针"a"从const [u64]转换为const [u8]。
// 打印指针"b"的值。
// 在unsafe块中，断言指针"b"指向的值的大小是否等于13。
