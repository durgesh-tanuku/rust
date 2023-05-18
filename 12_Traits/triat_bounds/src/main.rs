use std::any;

fn print_type<T: std::fmt::Debug>(item: T) {
    println!("{:?} is of type {}", item, std::any::type_name::<T>());
}
fn main() {
    print_type(3);
    print_type(3.0);
    print_type("three");
    print_type([13]);
}
