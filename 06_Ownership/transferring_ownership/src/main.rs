fn main() {
    let rocket_fuel = String::from("RP-1");
    // process_fuel(rocket_fuel.clone()); // This statement works here, but the modifications inside the function will not get reflected back
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("Processing propellant is {}", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}
