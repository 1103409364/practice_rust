pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        println!("left: {}, right: {}", left, right);
        let mid = (left + right) / 2;
        if mid == 0 && array[mid] != key {
            return None;
        }
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}
