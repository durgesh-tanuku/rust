fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let result = largest(a, b);
    println!("The largest number is {}", result);

    let c = 'a';
    let d = 'z';
    let result = largest(c, d);
    println!("The largest char is {}", result);
}
