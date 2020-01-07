use std::io; // provides useful inputting features

// entry function
fn main() {
    println!("Guess the number.");

    // variables immutable by default
    let mut guess = String::new(); // makes a new string
    io::stdin().read_line(&mut guess).expect("Failed to read line."); // will crash the program if there's an err type returned
    println!("You guessed: {}", guess);
}
