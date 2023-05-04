fn main() {
    let mut result = square(13);
    println!("result is {}", result);
    result = square1(13);
    println!("result is {}", result);
    let result1 = square2(13);
    println!("result is {:?}", result1);
}

/* return at end */
fn square(x: i32) -> i32 {
    println!("Squaring {}", x);
    x*x
}

/* return at middle */
fn square1(x: i32) -> i32 {
    println!("Squaring {}", x);
    return x*x;
    println!("End of function"); // warning: unreachable statement. But fine
}

/* return tuple */
fn square2(x: i32) -> (i32, i32) {
    (x, x*x)
}