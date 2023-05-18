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
}

fn main() {
    let mut s = Shuttle  {
        name: String::from("Endeavor"),
        crew_size: 10,
        propellant: 42344.0
    };

    s.name = String::from("Atlantis");

    println!("shuttle is {:?}", s);

    println!("name is {}", s.get_name());

    println!("fuel is {}", s.propellant);
    s.add_fuel(1000.0);
    println!("fuel is {}", s.propellant);
}