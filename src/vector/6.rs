// 容量 capacity 是已经分配好的内存空间，用于存储未来添加到 Vec 中的元素。而长度 len 则是当前 Vec 中已经存储的元素数量。如果要添加新元素时，长度将要超过已有的容量，那容量会自动进行增长：Rust 会重新分配一块更大的内存空间，然后将之前的 Vec 拷贝过去，因此，这里就会发生新的内存分配( 目前 Rust 的容量调整策略是加倍，例如 2 -> 4 -> 8 ..)。

// 若这段代码会频繁发生，那频繁的内存分配会大幅影响我们系统的性能，最好的办法就是提前分配好足够的容量，尽量减少内存分配。
// 修复错误
fn main() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...但是下面的代码会造成新的内存分配
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


    // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!")
}