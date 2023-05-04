fn main() {
    let x = 10; // Default type is 32 bit signed integer (i32)
    //let y: u8 = -20; // error: cannot apply unary operator `-` to type `u8`
    let mut y: u8 = 255; // Can only take 0 to 255
    //y = y + 1; // error: thread 'main' panicked at 'attempt to add with overflow', src/main.rs:5:9
    println!("x is {} and y is {}", x, y);
}
