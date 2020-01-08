use std::io; // provides useful inputting features
use std::cmp::Ordering; // provides the ordering enums
use rand::Rng; // provides random number generation

// entry function
fn main() {
    println!("Try to guess a secret number between 0 and 128");

    // variables immutable by default
    // secret_number is a string
    let secret_number = rand::thread_rng().gen_range(0, 128);

    let mut guesses: u8 = 0;
    loop {
        guesses += 1;

        let mut guess = String::new(); // makes a new mutable string

        println!("Guess the number.");

        // queries for response
        // note it passes a reference, not the value itself
        io::stdin().read_line(&mut guess).expect("Failed to read line."); // will crash the program if there's an err type returned

        // redefins guess
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Actually guess a valid number.");
                continue;
            },
        };
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) { // just pattern matching
            // pattern        statement run if matched
            Ordering::Less => {
                println!("Too small! Try again.");
            },
            Ordering::Greater => {
                println!("Too big! Try again.");
            },
            Ordering::Equal => {
                println!("You win!");

                // the brackets are necessary
                if (guesses != 1) {
                    println!("You have used {} guesses to guess the answer", guesses);
                } else {
                    println!("You have used 1 guess to guess the answer");
                }

                break;
            },
        }

    }
}
