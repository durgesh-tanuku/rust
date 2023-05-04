fn main() {
    let message = String::from("Message from Earth!");
    println!("message is {}", message);

    let last_word: &str = &message[13..];
    println!("last word is {}", last_word);

    let planets = [1,2,3,4,5,6,7,8];
    let inner_planets = &planets[..4];
    println!("inner planets are {:?}", inner_planets);

    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message[10..]);
    println!("first word is {}", first_word)
}

fn get_first_word(message: &str) -> &str {
    let letters = message.as_bytes();

    for (index, &letter) in letters.iter().enumerate() {
        if letter == b' ' {
                return &message[..index];
        }
    }

    &message
}
