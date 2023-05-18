#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallon: f64) {
        self.propellant += gallon;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 10,
            propellant: 42344.0
        }
    }
}

fn main() {
    let vehicle = Shuttle::new("Endeavour");

    println!("shuttle is {:?}", vehicle);
}