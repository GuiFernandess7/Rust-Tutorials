use std::cmp::Ordering;
use std::io;

fn main() {
    let mut age: String = String::new();
    println!("Type your age: ");
    io::stdin().read_line(&mut age).expect("Error reading");
    let age: i32 = age.trim().parse().unwrap();
    let voting_age: i32 = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You are now able to vote"),
    }
}
