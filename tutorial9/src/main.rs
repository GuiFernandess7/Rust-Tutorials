use std::io;

fn main() {
    // str slicing (Fixed length):
    let name: &str = "David";

    // Heap string or Dynamic String (String)
    let mut d_str = String::new(); // or String::from("")";
    d_str.push_str("r");
    d_str.push_str("u");
    d_str.push_str("s");
    d_str.push_str("t");
    println!("{d_str} from push method"); 

    // Join from iterable
    let letters = ["r", "u", "s", "t"];
    let all_together = String::from_iter(letters);
    println!("{all_together} from array");

    // Explicit coersion
    let s: String = "Rust".into();

    read_user_input();
}

fn read_user_input(){
    println!("{}", "-".repeat(20));
    let mut input = String::new();
    println!("Type a text");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading console");

    println!("You typed '{}' ", input);
    println!("Length of input: {}", input.trim().chars().count());
    println!("{}", "-".repeat(20));
}