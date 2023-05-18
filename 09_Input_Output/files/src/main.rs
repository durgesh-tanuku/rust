use std::fs;
use std::io::prelude::*;

fn main() {
    read_from_file();

    write_to_file();
}

fn read_from_file() {
    let planets  = fs::read_to_string("planets.txt").unwrap();
    println!("contents are {}", planets);

    for line in planets.lines() {
        println!("line is {}", line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents are {:?}", contents);
}

fn write_to_file() {
    let mut speech = String::new();
    speech.push_str("abc\n");
    speech.push_str("xyz");
    speech.push_str("dsgsgdsgs");
    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");
}
