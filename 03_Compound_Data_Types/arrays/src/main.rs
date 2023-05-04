fn main() {
    let letters = ['a', 'b', 'c'];
    let mut first_letter = letters[0];
    println!("first letter is {}", first_letter);
    first_letter = 'x';
    println!("first letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5]; // or [0,0,0,0,0]; 
    println!("last number is {}", numbers[4]);
    //println!("last number is {}", numbers[5]); // error: index out of bounds: the length is 5 but the index is 5
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index]); // compiles but run time error: thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5'
}
