use std::collections::{HashMap, HashSet};

/// # 解题思路
/// 这是一个字母算术问题（Alphametics）的求解器。在这种问题中，每个字母代表0-9之间的一个唯一数字。
/// 我们需要找到一种字母到数字的映射，使得等式成立。
///
/// 实现采用回溯法（Backtracking）：
/// 1. 解析输入的等式，提取加数和和数
/// 2. 确定所有唯一字母和每个单词的首字母（首字母不能为0）
/// 3. 检查是否有超过10个唯一字母（因为只有10个数字可用）
/// 4. 使用回溯法为每个字母尝试不同的数字分配
/// 5. 检查最终的分配是否使等式成立
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // 解析等式，按 " == " 分割
    let parts: Vec<&str> = input.split(" == ").collect();
    if parts.len() != 2 {
        return None;
    }

    let left_side = parts[0];
    let right_side = parts[1];

    // 提取所有加数，按 " + " 分割
    let addends: Vec<&str> = left_side.split(" + ").collect();
    let sum = right_side;

    // 收集所有唯一字母和首字母
    let mut all_letters = HashSet::new();
    let mut first_letters = HashSet::new();

    for word in addends.iter().chain(std::iter::once(&sum)) {
        let word = word.trim();
        if !word.is_empty() {
            // 将单词的所有字母添加到唯一字母集合中
            all_letters.extend(word.chars());
            // 记录单词的首字母，这些字母不能映射为0
            first_letters.insert(word.chars().next().unwrap());
        }
    }

    // 检查是否有超过10个唯一字母（因为只有0-9这10个数字可用）
    if all_letters.len() > 10 {
        return None;
    }

    // 将集合转换为向量，方便索引
    let letters: Vec<char> = all_letters.into_iter().collect();

    // 初始化回溯所需的状态
    let mut assignments = HashMap::new(); // 字母到数字的映射
    let mut used_digits = HashSet::new(); // 已使用的数字集合

    // 开始回溯搜索
    if backtrack(
        &letters,
        &first_letters,
        &addends,
        sum,
        0,
        &mut assignments,
        &mut used_digits,
    ) {
        Some(assignments) // 找到有效解决方案
    } else {
        None // 无解
    }
}

/// 回溯函数，尝试为每个字母分配一个数字
fn backtrack(
    letters: &[char],                    // 所有唯一字母
    first_letters: &HashSet<char>,       // 所有单词的首字母
    addends: &[&str],                    // 等式左侧的加数
    sum: &str,                           // 等式右侧的和
    index: usize,                        // 当前处理的字母索引
    assignments: &mut HashMap<char, u8>, // 字母到数字的当前映射
    used_digits: &mut HashSet<u8>,       // 已使用的数字集合
) -> bool {
    // 如果所有字母都已分配数字，检查等式是否成立
    if index == letters.len() {
        return is_valid_solution(addends, sum, assignments);
    }

    let letter = letters[index];

    // 尝试每个可能的数字(0-9)
    for digit in 0..=9 {
        // 如果数字已被使用，跳过
        if used_digits.contains(&digit) {
            continue;
        }

        // 如果是首字母且数字为0，跳过（避免前导零）
        if digit == 0 && first_letters.contains(&letter) {
            continue;
        }

        // 为当前字母分配数字
        assignments.insert(letter, digit);
        used_digits.insert(digit);

        // 递归处理下一个字母
        if backtrack(
            letters,
            first_letters,
            addends,
            sum,
            index + 1,
            assignments,
            used_digits,
        ) {
            return true; // 找到有效解决方案
        }

        // 回溯：移除当前分配
        assignments.remove(&letter);
        used_digits.remove(&digit);
    }

    // 所有可能性都尝试失败
    false
}

/// 检查当前分配是否使等式成立
fn is_valid_solution(
    addends: &[&str],                // 等式左侧的加数
    sum: &str,                       // 等式右侧的和
    assignments: &HashMap<char, u8>, // 字母到数字的映射
) -> bool {
    // 计算每个加数的值
    let addend_values: Vec<u64> = addends
        .iter()
        .map(|word| word_to_number(word, assignments))
        .collect();

    // 计算和的值
    let sum_value = word_to_number(sum, assignments);

    // 检查等式是否成立：所有加数之和等于和值
    addend_values.iter().sum::<u64>() == sum_value
}

/// 将单词转换为数字
fn word_to_number(word: &str, assignments: &HashMap<char, u8>) -> u64 {
    // 使用fold函数将字母依次转换为数字并组合成一个数
    // 对于每个字母，将累积值乘以10并加上该字母对应的数字
    word.chars()
        .fold(0, |acc, c| acc * 10 + assignments[&c] as u64)
}
