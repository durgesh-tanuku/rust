fn main() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
    }
    println!("Propellant is {}", propellant);
}
