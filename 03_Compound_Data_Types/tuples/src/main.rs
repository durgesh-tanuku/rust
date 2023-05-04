fn main() {
    let mut stuff = (2, 5.1, 'x');
    //let mut stuff: (u8, f32, char) = (2, 5.0, 'x'); // If want to store the data as non default data types.
    stuff.0 += 10;
    let first_item = stuff.0;

    println!("first item is {}", first_item);

    let (a, b, c) = stuff;
    println!("a is {}, b is {} and c is {}", a, b, c);
}
