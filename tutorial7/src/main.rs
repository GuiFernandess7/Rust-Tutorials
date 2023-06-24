fn main() {
    println!("Hello, world!");
    print_numbers_sum(25, 100);

    // The "let number" part is a statement
    // The rest is a expression
    let number = {
        let x = 3;
        x + 1 // Doesn't require semicolon because number var 
              // returns the result
    };
    println!("Result of scope variable: {}", number);

    let result = add_numbers(2, 3);
    println!("Result of returned value: {}", result);
}

fn print_numbers_sum(x: i64, y: i64){ 
    println!("Result of printed value {}", x + y);
}

fn add_numbers(x: i32, y: i32) -> i32{
    return x + y;
    // or
    // let result = x + y;
    // result
}
