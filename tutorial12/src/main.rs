use std::io;
use rand::Rng;

fn main() {
    println!("Type your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you!";
    io::stdin().read_line(&mut name).expect("Didn't received input");
    let age: &str = "25";
    let age: i32 = age.trim().parse().expect("Wasn't assigned a number");
    println!("Hello {}!, {}.I know you are {} years old", name.trim_end(), greeting, age);
    // trim_end removes the automatic new line when pressed enter

    let can_vote = if age >= 18 {
        true
    } else {
        false
    };

    println!("Can vote: {can_vote}");

    let random_num: i32 = rand::thread_rng().gen_range(1..85); // random number of 1 to 100
    // println!("A random number: {}", random_num);

    match random_num {
        50 | 100 => println!("The number is divided by 2 and 5 and ends with zero"),
        30 | 60 | 90 => println!("The number is divided by 3 and 10 and ends with zero"),
        25 | 75 => println!("The number is divided by 5 and 25 and ends with five"),
        _ => println!("Good Luck guessing the number lol"),
    };

    // Create number variable
    let mut number: String = String::new();
    println!("Guess a number: ");

    // Read the terminal
    io::stdin().read_line(&mut number).expect("Could not read value!");

    // Converts number selected and generates random number
    let number: i32 = number.trim_end().parse().unwrap();

    if number >= 1 && number <= 10 {
        println!("The number is between 1 and 10"); 
    } else if number > random_num {
        println!("Wrong! The number is {}", random_num);
    } else if number < random_num {
        println!("Wrong! The number is {}", random_num);
    } else {
        println!("Congratulations! You guessed the number {} correctly!", random_num);
    }

}
