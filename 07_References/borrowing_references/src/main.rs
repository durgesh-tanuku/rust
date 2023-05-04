fn main() {
    let rocket_fuel = String::from("RP-1");

    let (rocket_fuel, length) = process_fuel(rocket_fuel);
    println!("rocket fuel is {} and length is {}", rocket_fuel, length);

    let length = process_fuel1(&rocket_fuel);
    println!("rocket fuel is {} and length is {}", rocket_fuel, length);

    let mut rocket_fuel = String::from("LNG");
    let length = process_fuel2(&mut rocket_fuel);
    println!("rocket fuel is {} and length is {}", rocket_fuel, length);

    let rocket_fuel = produce_fuel();
    println!("rocket fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> (String, usize) {
    println!("propellant is {}", propellant);
    let length = propellant.len();
    (propellant, length)
}

fn process_fuel1(propellant: &String) -> usize {
    println!("propellant is {}", propellant);
    let length = propellant.len();
    length
}

fn process_fuel2(propellant: &mut String) -> usize {
    propellant.push_str("-1");
    println!("propellant is {}", propellant);
    let length = propellant.len();
    length 
}

fn produce_fuel() -> String {
    let new_fuel = String::from("LNG-2");
    new_fuel // returning borrowed reference will throw error as new_fuel gets out of scope at the end of produce_fuel. This is called dangling reference. 
}