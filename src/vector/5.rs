// 与 String 的切片类似， Vec 也可以使用切片。如果说 Vec 是可变的，那它的切片就是不可变或者说只读的，我们可以通过 & 来获取切片。

// 在 Rust 中，将切片作为参数进行传递是更常见的使用方式，例如当一个函数只需要可读性时，那传递 Vec 或 String 的切片 &[T] / &str 会更加适合。


// 修复错误
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // 越界访问将导致 panic.
    // 修改时必须使用 `v.len`
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // 切片是只读的
    // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..];
    // slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}