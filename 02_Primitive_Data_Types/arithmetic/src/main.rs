fn main() {
    let x = 10;
    let y = 3.0;
    let mut z = x as f64 / y; // x = 10.0, y = 3 --> error: no implementation for `{float} / {integer}`
    println!("z is {}", z);
}
