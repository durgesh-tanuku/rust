use std::io;

fn main() {
    let mut buffer = String::new();
	println!("Input a message");
	io::stdin().read_line(&mut buffer);
	println!("buffer is {}", buffer);
	buffer.clear();

	println!("Enter number");
	io::stdin().read_line(&mut buffer);
	println!("Buffer is {}", buffer);

	let mut num =  buffer.trim().parse::<i32>().unwrap();
	num = num + 1;
	println!("number is {}", num);
}
