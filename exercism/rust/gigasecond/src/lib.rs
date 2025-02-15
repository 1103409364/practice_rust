use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds（1000000000） after start.
pub fn after(start: DateTime) -> DateTime {
    let seconds_to_add = 1000000000;
    // 1. 创建一个 Duration 对象
    let duration = Duration::seconds(seconds_to_add);
    // 2. 使用 + 运算符将 Duration 添加到 PrimitiveDateTime
    start + duration
}
