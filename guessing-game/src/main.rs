// Bring the io library from std into scope
use std::io;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number!");

    loop {
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        // Create a new growable instance of a string
        // The string guess needs to be mutable since it's value is changed by the user.
        let mut guess = String::new();

        // & indicates this argument is a reference to `guess` rather than the value of `guess`
        io::stdin()
            .read_line(&mut guess) // <- readline() returns  io::Result (Either Ok or Err)
            .expect("Failed to Read Line"); // expect() will handle the io::Result (Err) if one is returned by read_line()

        // Rust allows for shadowing the previous value of guess with a new one.
        // This effectively allows for converting values from one type to another
        // trim() eliminates whitespace characters at beginning and end of the original string
        // parse() will parse a string into a number 
        // expect() will handle the std::Result (Err) if one is returned by parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        /*
        * Match expressions are made up of arms. An arm consists of a pattern and the code that should
        * be run if the value given to the beginning of the match expression fits that arm's pattern.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
