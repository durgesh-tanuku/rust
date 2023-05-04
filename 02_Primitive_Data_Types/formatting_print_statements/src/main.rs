fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;

    //print!("c is {}", c); // prints without new line

    println!("c is {}", c);      // c is 3.3333333333333335
    println!("c is {:.3}", c);   // c is 3.333
    println!("c is {:8.3}", c);  // c is    3.333
    println!("c is {:08.3}", c); // c is 0003.333

    println!("a is {0} and b is {1}", a, b);
}
