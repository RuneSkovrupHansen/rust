use rand::Rng;
use std::cmp::Ordering;
use std::io; // Bring io library into scope, could use without using syntax std::io::

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create mutable value, immutable by default.
        let mut guess = String::new();

        // String::new indicates that new is an associated function of the String type.

        io::stdin()
            .read_line(&mut guess) // Pass mutable reference to variable.
            .expect("Failed to read line");

        // read_line returns a Result value. An enum that can be one of multiple possible states. Each state is called a variant.
        // Variant of result is Ok and Err. Inside Ok is the successfully generated value, inside Err is info on how or why operation failed.

        // Use of shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Use of catchall "_" to handle all error cases
        };

        // 'match' made up of arms. An arm consists of a pattern match against.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {guess}");
    }
}
