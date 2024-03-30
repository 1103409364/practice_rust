mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    // lib hello_package 名称在 Cargo.toml 中被设置为 hello_package
    assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");
    // 使用 pub use 进行再导出
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
}
