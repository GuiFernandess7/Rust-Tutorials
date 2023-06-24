fn main() {
    let cond = 2 < 1;
    println!("2 is less than 1: {}", cond);
    let cond2 = (2 as f32) <= 2.2;
    println!("2 is less than 2.2: {}", cond2);

    // Logical operators: && || ! 
    let cond3 = true && cond; // - returns false 
    let cond4 = !(true || cond3); // - returns false
    println!("{}", cond4);

    // if statements
    let food = "pizza";
    if food == "cookie"{
        println!("{}", food);
    } else {
        println!("food is not cookie");
    }
}
