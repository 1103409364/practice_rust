pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;

        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => {
                if mid == 0 {
                    return None;
                }
                left = mid + 1
            }
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    return None;
                }
                right = mid - 1
            }
        }
    }
    None
}
