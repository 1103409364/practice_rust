// This is a simple example of using a stack to convert decimal to binary.
fn divide_by_two(mut dec_num: u32) -> String {
    println!("divide_by_two({})", dec_num);
    // 用栈来保存余数 rem
    let mut rem_stack = Vec::new(); // Stack::new();
    //余数rem入栈
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }
    // 栈中元素出栈组成字符串
    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}

fn main() {
    let bin_str: String = divide_by_two(10);
    println!("10 is b{bin_str}");
    // println!("hello, world!")
}
