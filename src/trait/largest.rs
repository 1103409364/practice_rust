fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // 直接访问 item。不会发生所有权转移。
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    // 尝试访问 numbers 列表。为什么没有报错？对于基本类型（如整数、浮点数、布尔值等），在 Rust 中不会发生所有权转移。
    println!("number_list is {:?}", number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
