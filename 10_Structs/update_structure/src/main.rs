#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let mut vehicle1 = Shuttle  {
        name: String::from("Endeavour"),
        crew_size: 10,
        propellant: 42344.0
    };

    vehicle1.name = String::from("Atlantis");

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle1
    };

    let vehicle3 = Shuttle {
        ..vehicle1.clone()   // without clone() compiler throws error: value borrowed here after partial move
    };

    println!("vehicle1 is {:?}", vehicle1);
    println!("vehicle2 is {:?}", vehicle2);
    println!("vehicle3 is {:?}", vehicle3)
}
