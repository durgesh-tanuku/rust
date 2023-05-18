#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble"),
        velocity: 56.7
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 36.9
    };

    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}
