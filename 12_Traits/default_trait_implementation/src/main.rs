use std::fmt::format;


struct Satellite {
    name: String,
    velocity: f64
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: i32
}

trait Describe {
    fn describe(&self) -> String {
        format!("An object flying through space")
    }
}

impl Describe for Satellite {
    
}

impl Describe for SpaceStation {
    fn describe(&self) -> String {
        format!("{} has {} crew members and located at height of {}", self.name, self.crew_size, self.altitude)
    }
}


fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Satellite"),
        velocity: 154.9
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 8,
        altitude: 8000
    };
    println!("satellite is {}", hubble.describe());
    println!("spacestation is {}", iss.describe());
}
