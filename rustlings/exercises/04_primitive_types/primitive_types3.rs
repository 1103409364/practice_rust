fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [1; 100]; // 长度 100，填充 1
    println!("{a:?}");
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
