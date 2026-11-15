use std::println;
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let random_number: u8 = rng.random();
    println!("Random number: {}", random_number);
    println!("Random number: {random_number}");
}
