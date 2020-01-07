use std::io; // provides useful inputting features
use rand::Rng; // provides random number generation

// entry function
fn main() {
    println!("Guess the number.");

    // variables immutable by default
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    let mut guess = String::new(); // makes a new mutable string

    // queries for response
    // note it passes a reference, not the value itself
    io::stdin().read_line(&mut guess).expect("Failed to read line."); // will crash the program if there's an err type returned

    println!("You guessed {}", guess);
}
