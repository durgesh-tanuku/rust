fn main() {
    let mut outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        /*outer_planet = inner_planet;
        println!("Inner planet is {}", inner_planet); // error: value borrowed here after move*/
        outer_planet = inner_planet.clone(); // creates another instance on heap
        println!("Inner planet is {}", inner_planet);
        inner_planet.clear(); // Changing the value doesn't affect the already cloned value to outer_planet
        println!("Inner planet is {}", inner_planet);
    }
    println!("Outer planet is {}", outer_planet);
}
