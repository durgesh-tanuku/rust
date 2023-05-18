#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let mut s = Shuttle  {
        name: String::from("Endeavor"),
        crew_size: 10,
        propellant: 42344.0
    };

    s.name = String::from("Atlantis");

    println!("shuttle is {:?}", s);
}
