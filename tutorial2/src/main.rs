fn main() {
    let x = 4; // immutable variable
    println!("x is: {}", x);
    let mut y = 5;
    println!("y is: {}", y);
    y = 10;
    println!("y is now {} because is a mutable variable", y);
    println!("-----------------");
    // making a new scope:
    {
        let x: i32 = x - 2;
        println!("x is: {}", x);
    }

    let x = 16; // overwrite variable x
    println!("x was overwritten and now is {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{} is a constante (var name:  const SECONDS_IN_MINUTE: u32 = 60;)", SECONDS_IN_MINUTE);

    let h = 0xff; // hexadecimal representation
    let byte = b'A'; // bytes
    println!("0xff value: {}", h);
    println!("byte of letter A: {}", byte);
}
