use std::result;

fn best_fuel<'a>(fuel1: &'a str, fuel2: &'a str) -> &'a str {
    if fuel1.len() > fuel2.len() {
        fuel1
    } else {
        fuel2
    }
}

fn main() {
    let rp1 = String::from("RP-1");
    let result;
    {
        let lng = String::from("LNG");
        result = best_fuel(&rp1, &lng);
    }
    println!("best fuel is {}", result);
}
