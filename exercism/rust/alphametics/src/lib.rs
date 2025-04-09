/// 这个模块实现了一个字母算术谜题（也称为密码算术）求解器。
/// 字母算术谜题是一种数学谜题，其中字母被用来表示数字。
/// 目标是找到一个字母到数字的映射，使得给定的算术等式成立。
/// 例如："SEND + MORE = MONEY"，其中每个字母代表一个唯一的数字。
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// 核心思想是在进行排列测试之前转换等式，以尽可能快地计算。
// 最简单的测试数据模型就是字母因子（系数）的列表，
//     这样因子乘以字母值并求和后，对于正确的解会得到0。
// 为此，我们需要遍历等式，根据字母在单词中的位置（x1, x10, x100等）
//     来累加和记住每个字母的因子。
// 在"=="之后，我们需要改变因子的符号，反向解析输入字符串会更方便。
// 对字母/因子进行排序也会影响性能，这是我发现的。
// 此外，我们必须检查找到的解是否符合"首位不能为0"的规则，
//     所以我们需要知道哪些字母出现在编码数字的第一位。

/// 计算等式中每个字母的因子。
///
/// # 参数
/// * `input` - 输入的等式字符串（例如："SEND + MORE = MONEY"）
///
/// # 返回值
/// 一个元组，包含：
/// * 等式中唯一字母的向量
/// * 每个字母对应的因子向量
///
/// 因子表示基于字母在等式中位置的系数。
/// 例如，在"SEND"中，S的因子是1000，E是100，N是10，D是1。得到 {S:1000,E:100,N:10,D:1}
fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    let mut sign = -1; // 从等式右侧开始，使用负号
    let mut pos = 0; // 位置乘数（1, 10, 100等）
    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1; // 切换到等式左侧，使用正号
                pos = 0 // 重置位置计数器
            }
            '+' => pos = 0, // 对每个项重置位置计数器
            _ => {
                *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }
    // 按因子的绝对值排序以提高性能
    factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip() // unzip 将一个元组转换为两个独立的迭代器
}

/// 解决字母算术谜题。
///
/// # 参数
/// * `input` - 输入的等式字符串（例如："SEND + MORE = MONEY"）
///
/// # 返回值
/// * `Some(HashMap<char, u8>)` - 如果找到解，返回字母到数字的映射
/// * `None` - 如果不存在解
///
/// # 示例
/// ```
/// use alphametics::solve;
///
/// let solution = solve("SEND + MORE = MONEY").unwrap();
/// assert_eq!(solution.get(&'S'), Some(&9));
/// ```
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // 找出所有出现在数字首位的字母
    let firsts = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();

    // 计算每个字母的因子 letters ["S", "E", "N", "D", "M", "O", "R", "Y"] factors [1000, 100, 10, 1, 1000, 100, 10, 1]
    let (letters, factors) = calc_factors(&input);

    // 尝试所有可能的数字排列
    for perm in (0..=9).permutations(letters.len()) {
        // 计算因子 * 数字的和
        let sum = perm
            .iter()
            .enumerate()
            .map(|(i, v)| v * factors.get(i).unwrap())
            .sum::<i64>();

        // 检查这个排列是否是有效解：
        // 1. 和应该为0（等式平衡）
        // 2. 首位数字不能为0
        if sum == 0
            && !perm
                .iter()
                .enumerate()
                .any(|(i, v)| *v == 0 && firsts.contains(letters.get(i).unwrap()))
        {
            // 创建并返回解映射 字母: 数字
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(i, v)| (*letters.get(i).unwrap(), *v as u8)),
            ));
        }
    }
    None
}
