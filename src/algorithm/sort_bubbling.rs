// 两两比较。两层循环
fn bubble_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    // for i in 1..nums.len() {
    //     for j in 0..nums.len() - i {
    //         if nums[j] > nums[j + 1] {
    //             nums.swap(j, j + 1)
    //         }
    //     }
    // }
    // let mut len = nums.len() - 1;
    // while len > 0 {
    //     for i in 0..len {
    //         if nums[i] > nums[i + 1] {
    //             nums.swap(i, i + 1);
    //         }
    //     }
    //     len -= 1;
    // }

    // let mut len = nums.len() - 1;
    // let mut mompare = true;
    // while len > 0 && compare {
    //     compare = false;
    //     for i in 0..len {
    //         if nums[i] > nums[i + 1] {
    //             nums.swap(i, i + 1);
    //             compare = true;  // 数据无序 ， 还需继续比较
    //         }
    //     }
    //     len -= 1;
    // }

    // i 后面的项不停跟第 i 项比较，交换位置。不需要处理边界下标值的排序算法。看起来像冒泡实际上不是
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j)
            }
        }
    }
}

fn main() {
    let mut arr = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
