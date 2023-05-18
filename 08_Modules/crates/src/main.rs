//quse rand;
//use rand::random;
use rand::prelude::*;

fn main() {
    let number = rand::random::<u8>();
    println!("number is {}", number);
}
