fn main() {
    let x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }

    /*if x as bool { // Doesn't work
        println!("True")
    }*/

    let a = 3;
    let b = 5;

    if a > b {
        println!("a is greather than b");
    } else if a < b {
        println!("a is less than b");
    } else {
        println!("a equal to b");
    }

    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};

    println!("x is {}", x)
}