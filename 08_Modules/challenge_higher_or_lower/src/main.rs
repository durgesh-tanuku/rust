use rand::prelude::*;
use std::io;

fn main() {
    let num: u8 = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Enter a number: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
        let number: u8 = input.trim().parse::<u8>().unwrap();
        if number < num {
            println!("Too less");
        } else if number > num {
            println!("Too high");
        } else {
            println!("Yes number is {}", num);
            break;
        }
    }
}
