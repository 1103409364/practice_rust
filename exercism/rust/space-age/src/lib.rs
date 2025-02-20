// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

// Duration 结构体用于表示时间长度，存储秒数
#[derive(Debug)]  // 为 Duration 实现 Debug trait，用于打印调试信息
pub struct Duration {
    seconds: u64,  // 使用无符号64位整数存储秒数
}

// 为 Duration 实现 From trait，允许从 u64 类型转换为 Duration
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }  // 将传入的秒数封装到 Duration 结构体中
    }
}

// 定义 Planet trait，所有行星类型都需要实现这个特征
pub trait Planet {
    // 计算在给定时间长度内，在该行星上经过了多少年
    fn years_during(d: &Duration) -> f64;
}

// 使用宏来批量创建行星结构体
macro_rules! make_struct {
    // $(...),* 表示可以接受零个或多个由逗号分隔的标识符
    ($($s_name:ident),*) => {
        $(
            // 为每个传入的标识符创建一个空结构体
            pub struct $s_name;
        )*
    };
}

// 使用宏来批量实现 Planet trait
macro_rules! impl_planet {
    // $(...)+ 表示一个或多个重复模式，每个模式包含类型和轨道周期
    (for $(($t:ty, $period:expr)),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                // 定义地球年对应的秒数：365.25天 * 24小时 * 60分 * 60秒
                const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
                // 计算公式：
                // 1. 将输入的秒数转换为 f64
                // 2. 除以地球年的秒数得到地球年数
                // 3. 除以行星的轨道周期得到该行星上的年数
                d.seconds as f64 / EARTH_YEAR_IN_SECONDS / $period
            }
        })*
    }
}

// 使用 make_struct 宏创建所有行星的结构体
make_struct!(Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);

// 使用 impl_planet 宏为所有行星实现 Planet trait
// 每个行星都指定其轨道周期（相对于地球年）
impl_planet!(for
    (Mercury, 0.2408467),   // 水星轨道周期约为地球的 0.24 年
    (Venus, 0.61519726),    // 金星轨道周期约为地球的 0.62 年
    (Earth, 1.0),           // 地球轨道周期作为基准，为 1 年
    (Mars, 1.8808158),      // 火星轨道周期约为地球的 1.88 年
    (Jupiter, 11.862615),   // 木星轨道周期约为地球的 11.86 年
    (Saturn, 29.447498),    // 土星轨道周期约为地球的 29.45 年
    (Uranus, 84.016846),    // 天王星轨道周期约为地球的 84.02 年
    (Neptune, 164.79132)    // 海王星轨道周期约为地球的 164.79 年
);
