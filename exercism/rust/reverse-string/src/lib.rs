pub fn reverse(input: &str) -> String {
    // .collect(): 迭代器方法，它将迭代器中的所有元素收集到一个新的集合中。在这种情况下，因为返回类型是 String，collect() 会将反转后的字符序列组合成一个新的 String。
    input.chars().rev().collect()
}
