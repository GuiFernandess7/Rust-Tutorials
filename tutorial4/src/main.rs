// Preludes
use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new(); // Creating a empty string

    io::stdin().read_line(&mut input).expect("Failed to read line"); // Creating a mutable input
    println!("{}", input);
    // "&mut" means that the input written by the user is the same as the input variable
}
