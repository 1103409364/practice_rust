// 在数组中使用特征对象
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    // 将数组标记为特征对象数组。dny 只能出现在声明中，不能单独使用 dyn
    // 1 通过Box<T> 智能指针创建特征对象
    // let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];
    // 2 通过 & 引用创建特征对象
    let birds: Vec<&dyn Bird> = vec![&Duck, &Swan];

    for bird in birds {
        bird.quack();
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }
}
