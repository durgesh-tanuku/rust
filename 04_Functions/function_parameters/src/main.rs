fn main() {
    say_hello();
    say_num(45);
    let x = 1; // By default the type of x and y should be i32, but these are used in say_sum they will be declared as u8
    let y = 3;
    say_sum(x, y);
    say_num(x as i32);
}

fn say_hello() {
    println!("Hello!");
}

fn say_num(number: i32) {
    println!("number is {}", number);
}

fn say_sum(a: u8, b: u8) {
    println!("sum is {}", a+b)
}