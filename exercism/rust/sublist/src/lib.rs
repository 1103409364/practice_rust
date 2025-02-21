#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // todo!("Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}.");
    // 最大数
    let fist_string = first_list
        .iter()
        .map(|&x| format!("{:010}", x.to_string())) // 补齐，i32 最大 10 位
        .collect::<Vec<String>>()
        .join(","); // i32 数组不能直接使用 join 方法，转为 String
    let second_string = second_list
        .iter()
        .map(|&x| format!("{:010}", x.to_string()))
        .collect::<Vec<String>>()
        .join(",");
    // String 的 find 方法在字符串中查找子字符串，并返回一个 Option<usize>
    if let Some(_) = fist_string.find(&second_string) {
        if fist_string == second_string {
            return Comparison::Equal;
        }
        return Comparison::Superlist;
    }

    if let Some(_) = second_string.find(&fist_string) {
        if fist_string == second_string {
            return Comparison::Equal;
        }
        return Comparison::Sublist;
    }
    Comparison::Unequal
}
// 更好的解法：使用数组的 windows（滑动窗口） 和 any 方法。

// rust 中素组 == 比较，比较的是内容，而不是地址。
// 两个数组相等的条件：
// 长度相等：两个数组必须具有相同的长度。
// 元素类型相同：两个数组中的元素必须是相同的类型。
// 元素值相等：两个数组中对应位置的元素必须相等。这意味着对于每个索引 i，arr1[i] == arr2[i] 必须成立。
