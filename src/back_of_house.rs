// in src/back_of_house.rs
// 拆分方式2 同级目录里创建一个与模块（目录）同名的 rs 文件作为模块
use crate::front_of_house;
pub fn fix_incorrect_order() {
    cook_order();
    front_of_house::serving::serve_order();
}

pub fn cook_order() {}
