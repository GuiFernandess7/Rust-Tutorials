// Ownership & Borrowing
fn main() {
    // Copy Attribution (Atribuição por cópia)
    let a = 1; 
    let b = a; // B becomes a copy a A (Different allocation)

    // Reference Attribution (Atribuição por referência)
    let a = 1;
    let b = &a; // B points to A in memory (Same allocation)

    // Ownership (Posse/Propriedade)
    let a2 = String::from("Rust"); // Mutable value allocated in heap memory
    //let b2 = a2; // B2 DOES NOT become a copy of A2 because the value it is a muttable structure
    // Rust precisa ser performático e evita gastar energia
    //println!("{}", a2); // Gives an error because the valus was passed to b2

    // Solved: (Borrowing)
    let b2 = &a2; // a2 borrowed value to b2
    println!("{}", a2); 

    let mut name = "Username".to_string();
    to_uppercase(&mut name);
    println!("{name}")

}

fn to_uppercase(text: &mut String){ // Has to be mutable to be altered
    *text = text.to_uppercase(); 
}

