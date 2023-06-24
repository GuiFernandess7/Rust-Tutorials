// Calculator:
use std::io;

// Separate chars in input into vector
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading console");

    fn clean(c: &str) -> &str {
        c.trim()
    }

    //let nums: Vec<&str> = input.split(",")
    //    .map(clean) // Applying function to all items
    //    .collect();

    // OR

    let nums: Vec<i32> = input
        .split(",")
        .map(|c| c.trim().parse::<i32>().expect("Error parsing integer")) // Convertendo para i32 e tratando possíveis erros
        .collect();
    
    let result: i32 = nums.iter().sum();

    println!("The sum total is {}", result);
}
