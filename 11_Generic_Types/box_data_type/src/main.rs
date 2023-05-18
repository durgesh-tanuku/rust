use std::mem;

#[derive(Debug)]
struct Satellite {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let vehicle = Satellite {
        name: String::from("Hubble"),
        crew_size: 7,
        propellant: 56745.0
    };
    println!("vehicle is {:?}", vehicle);
    println!("vehicle size on stack is {}", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Satellite> = Box::new(vehicle);
    println!("boxed_vehicle size on stack is {}", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap is {}", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Satellite = *boxed_vehicle;
    println!("unboxed_vehicle size on stack is {}", mem::size_of_val(&unboxed_vehicle));
}
