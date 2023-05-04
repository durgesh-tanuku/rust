fn main() {
    let mut count = 0;
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("After the loop");
    println!("result is {}", result);

    /* While Loop */
    count = 0;
    let letters = ['a', 'b', 'c'];
    while count < letters.len() {
        println!("count is {}", count);
        println!("letter is {}", letters[count]);
        count += 1;
    }
    println!("After while loop");

    /* For Loop */
    let message = ['h', 'e', 'l', 'l', 'o'];
    for item in message.iter() {
        println!("item is {}", item);
    }

    for (index, item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
    }

    for number in 0..5 {
        println!("number is {}", number)
    }

    let matrix = [[10, 20, 30],
                  [40, 50, 60],
                  [70, 80, 90]];
    for row in matrix.iter() {
        for item in row.iter() {
            print!("{}\t", item);
        }
        println!();
    }
}
