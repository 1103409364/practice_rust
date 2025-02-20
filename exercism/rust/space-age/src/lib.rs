// Duration 结构体表示以地球年为单位的时间
pub struct Duration(f64);

// 实现从 u64 (秒) 转换到 Duration (年) 的转换
impl From<u64> for Duration {
    // 将秒转换为年 (1年 = 31557600秒)
    fn from(s: u64) -> Self {
        Duration((s as f64) / (31557600 as f64))
    }
}

// Planet trait 定义了行星的公共接口
pub trait Planet {
    // 返回行星的轨道周期（以地球年为单位）
    fn period() -> f64;

    // 计算在给定时间内行星绕太阳运转的圈数
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::period()
    }
}

// 宏用于批量创建行星结构体和实现 Planet trait
// 使用重复模式 ($...), * 来处理多个行星定义
macro_rules! planets {
    // $n:ident 是行星名称标识符，$p:expr 是轨道周期表达式
    // 模式 ($(...),*) 允许传入多个 name, period 对
    ($($n:ident, $p:expr),*) => {
        $(
            // 为每个行星创建空结构体
            pub struct $n;
            // 为每个行星实现 Planet trait
            impl Planet for $n {
                fn period() -> f64 { $p }
            }
        )*  // * 表示重复展开前面的代码块
    };
}

// 使用 planets! 宏一次性定义所有太阳系行星
// 每行包含 行星名, 轨道周期(以地球年为单位)
planets!(
    Earth, 1.0, // 地球: 基准周期
    Mercury, 0.2408467, // 水星: 约0.24地球年
    Venus, 0.61519726, // 金星: 约0.62地球年
    Mars, 1.8808158, // 火星: 约1.88地球年
    Jupiter, 11.862615, // 木星: 约11.86地球年
    Saturn, 29.447498, // 土星: 约29.45地球年
    Uranus, 84.016846, // 天王星: 约84.02地球年
    Neptune, 164.79132 // 海王星: 约164.79地球年
);
