// Added the attribute `macro_use` attribute.
#[macro_use]
mod macros {
    // 或者使用 #[macro_export] #[macro_export]: 这个属性告诉 Rust 编译器，这个宏应该被导出，以便在其他模块中使用。如果没有这个属性，宏将只能在定义它的模块内部使用。
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
