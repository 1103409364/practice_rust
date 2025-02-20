// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
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
    // 添加轨道周期参数 ty 类型参数 expr 表达式，, 分隔符 + 一个或多个
    (for $(($t:ty, $period:expr)),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                // 地球年换算成秒: 31557600 = 365.25 * 24 * 60 * 60
                const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
                // 将Duration中的秒数转换为f64，然后除以地球年秒数，再除以轨道周期
                d.seconds as f64 / EARTH_YEAR_IN_SECONDS / $period
            }
        })*
    }
}

make_struct!(Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
// 使用新的宏语法，为每个行星指定其轨道周期（相对于地球年）
impl_planet!(for
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
