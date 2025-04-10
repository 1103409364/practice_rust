use std::borrow::Borrow;

pub fn find<T, K>(array: impl AsRef<[T]>, key: K) -> Option<usize>
where
    T: Ord,
    K: Borrow<T>,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        println!("left: {}, right: {}, mid: {}", left, right, mid);
        match array[mid].cmp(key.borrow()) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => {
                if mid == 0 && right == 0 {
                    return None;
                }
                left = mid + 1
            }
            std::cmp::Ordering::Greater => {
                if mid == 0 && left == 0 {
                    return None;
                }
                right = mid - 1
            }
        }
    }
    None
}
