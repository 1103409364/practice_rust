// in lib.rs
// 将模块拆分到文件中

// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         pub fn seat_at_table() -> String {
//             String::from("sit down please")
//         }
//     }

//     pub mod serving {
//         pub fn take_order() {}

//         pub fn serve_order() {}

//         pub fn take_payment() {}

//         // 我猜你不希望顾客听到你在抱怨他们，因此让这个函数私有化吧
//         fn complain() {}
//     }
// }

// pub fn eat_at_restaurant() -> String {
//     front_of_house::hosting::add_to_waitlist();

//     back_of_house::cook_order();

//     String::from("yummy yummy!")
// }

// pub mod back_of_house {
//     pub fn fix_incorrect_order() {
//         cook_order();
//         crate::front_of_house::serving::serve_order();
//     }

//     pub fn cook_order() {}
// }

mod back_of_house;
mod front_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
