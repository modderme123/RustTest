use std::io; // provides useful inputting features
use std::cmp::Ordering; // provides the ordering enums
use rand::Rng; // provides random number generation

enum TestEnum {

    Less,
    Grater,
    Equal,

}

struct TestStruct {

    x: u32,
    y: String,

}
impl TestStruct {
    fn test(&mut self){
        self.x += 1;
    }
}

// entry function
fn main() {
    let mut x = TestStruct{x: 1, y: "hi".to_string()};
    x.test();
    dbg!(x.x);

    println!("Please try to guess the number.");

    // variables immutable by default
    // secret_number is a string
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    let mut guess = String::new(); // makes a new mutable string

    // queries for response
    // note it passes a reference, not the value itself
    io::stdin().read_line(&mut guess).expect("Failed to read line."); // will crash the program if there's an err type returned

    // redefins guess
    let guess : u32 = guess.trim().parse().expect("Please type an actual number.");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) { // just pattern matching
        // pattern        statement run if matched
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}
