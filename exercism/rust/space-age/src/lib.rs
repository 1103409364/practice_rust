// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        todo!("s, measured in seconds: {s}")
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// 创建结构体，传入
macro_rules! make_struct {
    // $(...) 重复匹配 ident 标识符 * 匹配零个或多个 ident 标识符
    ($($s_name:ident),*) => {
        $(
            pub struct $s_name;
        )*
    };
}

macro_rules! impl_planet {
    // for 分隔符，自定义。ty 类型, + 一个或多个
    (for $($t:ty),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
            }
        })*
    }
}

make_struct!(Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
// for 占位，可以不要
impl_planet!(for Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
