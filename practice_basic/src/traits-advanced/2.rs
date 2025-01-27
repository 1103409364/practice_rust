// 当我们使用泛型类型参数时，可以为该泛型参数指定一个具体的默认类型，这样当实现该特征时，如果该默认类型可以使用，那用户再无需手动指定具体的类型。

use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T = u8> {
    x: T,
    y: T,
}

// 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// 使用默认的泛型参数 Point<T> 传给 Sub
// impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// 使用默认泛型参数 Self
// impl<T: Sub<Output = T>> Sub<Self> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
fn main() {
    assert_eq!(
        Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 }
    );

    println!("Success!")
}
