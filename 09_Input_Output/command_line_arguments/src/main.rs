use std::env;

fn main() {
    if env::args().len() <= 3 {
        println!("Program need atleast 3 arguments");
        return;
    }

    for arg in env::args() {
        println!("{}\t", arg);
    }
    println!();

    for (index, arg) in env::args().enumerate() {
        println!("index {} has argument {}", index, arg);
    }

    let num = env::args().nth(2).unwrap();
    println!("2nd argument is {}", num);
}
