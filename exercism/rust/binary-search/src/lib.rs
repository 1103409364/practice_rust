use std::borrow::Borrow;

/// 在有序数组中执行二分查找
///
/// 参数说明:
/// - array: 要搜索的有序数组或切片的引用
/// - key: 要查找的目标值
///
/// 返回值:
/// - Some(usize): 如果找到目标值，返回其索引位置
/// - None: 如果未找到目标值
///
/// 类型约束:
/// - T: 数组元素类型，必须实现 Ord 特征以支持比较
/// - K: 目标值类型，必须可以借用为 T 类型
pub fn find<T, K>(array: impl AsRef<[T]>, key: K) -> Option<usize>
where
    T: Ord,
    K: Borrow<T>,
{
    // 将输入转换为切片以便操作
    let array = array.as_ref();
    // 如果数组为空，直接返回 None
    if array.is_empty() {
        return None;
    }

    // 初始化搜索边界
    let mut left = 0;
    let mut right = array.len() - 1;

    // 执行二分查找循环
    while left <= right {
        // 计算中间位置
        let mid = (left + right) / 2;
        println!("左边界: {}, 右边界: {}, 中间位置: {}", left, right, mid);

        // 比较中间元素与目标值
        match array[mid].cmp(key.borrow()) {
            // 找到目标值，返回其索引
            std::cmp::Ordering::Equal => return Some(mid),
            // 中间元素小于目标值，在右半部分继续查找
            std::cmp::Ordering::Less => {
                // 处理边界情况，防止无限循环
                if mid == 0 && right == 0 {
                    return None;
                }
                left = mid + 1
            }
            // 中间元素大于目标值，在左半部分继续查找
            std::cmp::Ordering::Greater => {
                // 处理边界情况，防止无限循环
                if mid == 0 && left == 0 {
                    return None;
                }
                right = mid - 1
            }
        }
    }
    // 未找到目标值，返回 None
    None
}
