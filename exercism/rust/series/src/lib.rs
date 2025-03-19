pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len) // 当 windows 方法的参数（即窗口大小 len）超过切片的最大长度时，windows 方法会返回一个空迭代器
        .map(|w| w.iter().collect())
        .collect()
}
