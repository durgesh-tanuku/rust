#[derive(Debug)]
struct Rectangle<T, U> {
    length: T,
    width: U
}

impl<T,U> Rectangle<T,U> {
    fn get_length(&self) -> &T {
        return &self.length;
    }
}

fn main() {
    let rect = Rectangle {
        length: 54u8,
        width: 64u16
    };
    println!("rect is {:?}", rect);
    println!("length is {}", rect.get_length());
}
