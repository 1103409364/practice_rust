//! 为 Crates root 的子模块添加注释

/// `add_to_waitlist` 函数用于将顾客添加到预约名单中。
///

// in src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}

pub fn seat_at_table() -> String {
    String::from("sit down please")
}
