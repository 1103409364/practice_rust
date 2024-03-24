// Rust 编译器需要知道一个函数的返回类型占用多少内存空间。由于特征的不同实现类型可能会占用不同的内存，因此通过 impl Trait 返回多个类型是不被允许的，但是我们可以返回一个 dyn 特征对象来解决问题。

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    // 填空
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");

    let swan = Swan;
    // swan 可以 fly，变成 bird 之后不能 fly。只能调用特征对象 Bird 上的方法
    swan.fly();

    println!("Success!")
}

// 实现以下函数
fn hatch_a_bird(num: i32) -> Box<dyn Bird> {
    if num == 2 {
        Box::new(Duck)
    } else {
        Box::new(Swan)
    }
}
