#[derive(Debug)]
struct Rectangle<T> {
    length: T,
    width: T
}

#[derive(Debug)]
struct Person<T, U> {
    salary: T,
    bank_balance: U
}

fn main() {
    let shape = Rectangle {
        length: 5.0,
        width: 45.0
    };

    println!("shape is {:?}", shape);

    let person = Person {
        salary: 20000,
        bank_balance: 50.0
    };

    println!("person details is {:?}", person);
}
