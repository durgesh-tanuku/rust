fn main() {
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("a NOT is {} and b NOT is {}", !a, !b);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    //let c = (a ^ b) | panic!(); // gives error as panic macro gets executed.
    let c = (a ^ b) || panic!(); // don't give error as short circuit logical OR will not execute panic macro.
    println!("c is {}", c);
}
