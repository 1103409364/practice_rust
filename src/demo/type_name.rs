fn print_type_of<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>())
}
fn main() {
    let x: u32 = 1;
    print_type_of(&x);
    let  my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    print_type_of(&prints_string);
}
