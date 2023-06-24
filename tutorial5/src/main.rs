use std::io;

fn main() {
    // If variables are from different types, operations won't work
    let x = 126.0 as f32;
    let z = x as i64; // convert x from f32 to i64
    let y: f64 = 10.0; 
    let max = (i32::MAX as i64) + 1;

    println!("x as f32: {}", x);
    println!("x is now of type i64");
    println!("Max number of type i32: {}", max);

    let mut input = String::new();
    println!("Type a number: {}", input);
    io::stdin().read_line(&mut input).expect("Catching error");

    let int_input: i64 = input.trim().parse().unwrap();
    // Trim cuts invisible or empty characters from terminal
    // parse tries to convert the string to i64 if possible
    // unwrap ajusts the conversion of i64 
    println!("{} + 2 = {}", int_input, int_input + 2);

}
