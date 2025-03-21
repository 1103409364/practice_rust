pub fn egg_count(display_value: u32) -> usize {
    // format!("{:b}", display_value)
    //     .chars()
    //     .fold(0, |acc, c| acc + if c == '1' { 1 } else { 0 })
    display_value.count_ones() as usize
    // count_ones 方法用于计算整数类型中二进制表示中 1 的个数（也称为 population count 或 Hamming weight）。这个方法在各种场景下都很有用，例如：
    // 位运算优化： 在某些算法中，需要快速计算一个数中 1 的个数。
    // 数据压缩： 可以用于评估数据的稀疏性。
    // 错误检测和纠正： 汉明重量与汉明距离有关，汉明距离可以用来衡量两个等长字符串之间的差异。
}
